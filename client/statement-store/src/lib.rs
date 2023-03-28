// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Disk-backed statement store.

#![warn(missing_docs)]
#![warn(unused_extern_crates)]

mod metrics;

pub use sp_statement_store::{Error, StatementStore};

use metrics::MetricsLink as PrometheusMetrics;
use parking_lot::RwLock;
use prometheus_endpoint::Registry as PrometheusRegistry;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::{hexdisplay::HexDisplay, Decode, Encode};
use sp_runtime::traits::Block as BlockT;
use sp_statement_store::{
	runtime_api::{InvalidStatement, StatementSource, ValidStatement, ValidateStatement},
	AccountId, BlockHash, Channel, DecryptionKey, Hash, NetworkPriority, Proof, Result, Statement,
	SubmitResult, Topic,
};
use std::{
	collections::{BTreeMap, HashMap, HashSet},
	sync::Arc,
};

const KEY_VERSION: &[u8] = b"version".as_slice();
const CURRENT_VERSION: u32 = 1;

const LOG_TARGET: &str = "statement-store";

const PURGE_AFTER: u64 = 2 * 24 * 60 * 60; //48h
const MAX_LIVE_STATEMENTS: usize = 8192;
const MAX_TOTAL_SIZE: usize = 64 * 1024 * 1024;

/// Suggested maintenance period. A good value to call `Store::maintain` with.
#[allow(dead_code)]
pub const MAINTENANCE_PERIOD: std::time::Duration = std::time::Duration::from_secs(30);

mod col {
	pub const META: u8 = 0;
	pub const STATEMENTS: u8 = 1;
	pub const EXPIRED: u8 = 2;

	pub const COUNT: u8 = 3;
}

#[derive(PartialEq, Eq)]
struct PriorityKey {
	hash: Hash,
	priority: u32,
}

#[derive(PartialEq, Eq)]
struct ChannelEntry {
	hash: Hash,
	priority: u32,
}

#[derive(Default)]
struct StatementsForAccount {
	// Statements ordered by priority.
	by_priority: BTreeMap<PriorityKey, (Option<Channel>, usize)>,
	// Channel to statement map. Only one statement per channel is allowed.
	channels: HashMap<Channel, ChannelEntry>,
	// Sum of all `Data` field sizes.
	data_size: usize,
}

impl PartialOrd for PriorityKey {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		Some(self.priority.cmp(&other.priority).then_with(|| self.hash.cmp(&other.hash)))
	}
}

impl Ord for PriorityKey {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		self.priority.cmp(&other.priority).then_with(|| self.hash.cmp(&other.hash))
	}
}

#[derive(Default)]
struct Index {
	by_topic: HashMap<Topic, HashSet<Hash>>,
	by_dec_key: HashMap<DecryptionKey, HashSet<Hash>>,
	statement_topics: HashMap<Hash, ([Option<Topic>; 4], Option<DecryptionKey>)>,
	entries: HashMap<Hash, (AccountId, u32, u32)>, /* Statement hash -> (Account id,
	                                                * global_priority, priority) */
	expired: HashMap<Hash, u64>, // Value is expiration timestamp.
	accounts: HashMap<AccountId, StatementsForAccount>,
	by_global_priority: BTreeMap<PriorityKey, usize>,
	max_entries: usize,
	max_size: usize,
	total_size: usize,
}

struct ClientWrapper<Block, Client> {
	client: Arc<Client>,
	_block: std::marker::PhantomData<Block>,
}

impl<Block, Client> ClientWrapper<Block, Client>
where
	Block: BlockT,
	Block::Hash: From<BlockHash>,
	Client: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
	Client::Api: ValidateStatement<Block>,
{
	fn validate_statement(
		&self,
		block: Option<BlockHash>,
		source: StatementSource,
		statement: Statement,
	) -> std::result::Result<ValidStatement, InvalidStatement> {
		let api = self.client.runtime_api();
		let block = block.map(Into::into).unwrap_or_else(|| {
			// Validate against the finalized state.
			self.client.info().finalized_hash
		});
		match api.validate_statement(block, source, statement) {
			Ok(r) => r,
			Err(_) => Err(InvalidStatement::InternalError),
		}
	}
}

/// Statement store.
pub struct Store {
	db: parity_db::Db,
	index: RwLock<Index>,
	validate_fn: Box<
		dyn Fn(
				Option<BlockHash>,
				StatementSource,
				Statement,
			) -> std::result::Result<ValidStatement, InvalidStatement>
			+ Send
			+ Sync,
	>,
	// Used for testing
	time_override: Option<u64>,
	metrics: PrometheusMetrics,
}

#[derive(Encode, Decode, Clone)]
struct StatementMeta {
	global_priority: u32,
}

#[derive(Encode, Decode)]
struct StatementWithMeta {
	meta: StatementMeta,
	statement: Statement,
}

enum IndexQuery {
	Unknown,
	Exists,
	Expired,
}

enum MaybeInserted {
	Inserted(HashSet<Hash>),
	Ignored,
}

impl Index {
	fn new() -> Index {
		Index { max_entries: MAX_LIVE_STATEMENTS, max_size: MAX_TOTAL_SIZE, ..Default::default() }
	}

	fn insert_new(
		&mut self,
		hash: Hash,
		account: AccountId,
		global_priority: u32,
		statement: &Statement,
	) {
		let mut all_topics = [None; 4];
		let mut nt = 0;
		while let Some(t) = statement.topic(nt) {
			self.by_topic.entry(t).or_default().insert(hash);
			all_topics[nt] = Some(t);
			nt += 1;
		}
		let key = statement.decryption_key();
		if let Some(k) = &key {
			self.by_dec_key.entry(k.clone()).or_default().insert(hash);
		}
		if nt > 0 || key.is_some() {
			self.statement_topics.insert(hash, (all_topics, key));
		}
		let priority = statement.priority().unwrap_or(0);
		self.entries.insert(hash, (account.clone(), global_priority, priority));
		self.by_global_priority.insert(
			PriorityKey { hash: hash.clone(), priority: global_priority },
			statement.data_len(),
		);
		self.total_size += statement.data_len();
		let mut account_info = self.accounts.entry(account).or_default();
		account_info.data_size += statement.data_len();
		if let Some(channel) = statement.channel() {
			account_info.channels.insert(channel, ChannelEntry { hash, priority });
		}
		account_info
			.by_priority
			.insert(PriorityKey { hash, priority }, (statement.channel(), statement.data_len()));
	}

	fn query(&self, hash: &Hash) -> IndexQuery {
		if self.entries.contains_key(hash) {
			return IndexQuery::Exists
		}
		if self.expired.contains_key(hash) {
			return IndexQuery::Expired
		}
		IndexQuery::Unknown
	}

	fn insert_expired(&mut self, hash: Hash, timestamp: u64) {
		self.expired.insert(hash, timestamp);
	}

	fn is_expired(&self, hash: &Hash) -> bool {
		self.expired.contains_key(hash)
	}

	fn iter(
		&self,
		key: Option<DecryptionKey>,
		topics: &[Topic],
		mut f: impl FnMut(&Hash) -> Result<()>,
	) -> Result<()> {
		let empty = HashSet::new();
		let mut sets: [&HashSet<Hash>; 4] = [&empty; 4];
		if topics.len() > 4 {
			return Ok(())
		}
		for (i, t) in topics.iter().enumerate() {
			let set = self.by_topic.get(t);
			if set.map(|s| s.len()).unwrap_or(0) == 0 {
				// At least one of the topics does not exist in the index.
				return Ok(())
			}
			sets[i] = set.expect("Function returns if set is None");
		}
		let sets = &mut sets[0..topics.len()];
		if sets.is_empty() && key.is_none() {
			// Iterate all entries
			for h in self.entries.keys() {
				log::trace!(target: LOG_TARGET, "Iterating: {:?}", HexDisplay::from(h));
				f(h)?
			}
		} else {
			// Start with the smallest topic set or the key set.
			sets.sort_by_key(|s| s.len());
			if let Some(key) = key {
				let key_set =
					if let Some(set) = self.by_dec_key.get(&key) { set } else { return Ok(()) };
				for item in key_set {
					if sets.iter().all(|set| set.contains(item)) {
						log::trace!(
							target: LOG_TARGET,
							"Iterating by key: {:?}",
							HexDisplay::from(item)
						);
						f(item)?
					}
				}
			} else {
				for item in sets[0] {
					if sets[1..].iter().all(|set| set.contains(item)) {
						log::trace!(
							target: LOG_TARGET,
							"Iterating by topic: {:?}",
							HexDisplay::from(item)
						);
						f(item)?
					}
				}
			}
		}
		Ok(())
	}

	fn maintain(&mut self, current_time: u64) -> Vec<Hash> {
		// Purge previously expired messages.
		let mut purged = Vec::new();
		self.expired.retain(|hash, timestamp| {
			if *timestamp + PURGE_AFTER <= current_time {
				purged.push(hash.clone());
				log::trace!(target: LOG_TARGET, "Purged statement {:?}", HexDisplay::from(hash));
				false
			} else {
				true
			}
		});
		purged
	}

	fn make_expired(&mut self, hash: &Hash, current_time: u64) -> bool {
		if let Some((account, global_priority, priority)) = self.entries.remove(hash) {
			let key = PriorityKey { hash: hash.clone(), priority: global_priority };
			let len = self.by_global_priority.remove(&key).unwrap_or(0);
			self.total_size -= len;
			if let Some((topics, key)) = self.statement_topics.remove(hash) {
				for t in topics {
					if let Some(t) = t {
						if let Some(set) = self.by_topic.get_mut(&t) {
							set.remove(hash);
						}
					}
				}
				if let Some(k) = key {
					if let Some(set) = self.by_dec_key.get_mut(&k) {
						set.remove(hash);
					}
				}
			}
			self.expired.insert(hash.clone(), current_time);
			if let std::collections::hash_map::Entry::Occupied(mut account_rec) =
				self.accounts.entry(account)
			{
				let key = PriorityKey { hash: hash.clone(), priority };
				if let Some((channel, len)) = account_rec.get_mut().by_priority.remove(&key) {
					account_rec.get_mut().data_size -= len;
					if let Some(channel) = channel {
						account_rec.get_mut().channels.remove(&channel);
					}
				}
				if account_rec.get().by_priority.is_empty() {
					account_rec.remove_entry();
				}
			}
			log::trace!(target: LOG_TARGET, "Expired statement {:?}", HexDisplay::from(hash));
			true
		} else {
			false
		}
	}

	fn insert(
		&mut self,
		hash: Hash,
		statement: &Statement,
		account: &AccountId,
		validation: &ValidStatement,
		current_time: u64,
	) -> MaybeInserted {
		let statement_len = statement.data_len();
		if statement_len > validation.max_size as usize {
			log::debug!(
				target: LOG_TARGET,
				"Ignored oversize message: {:?} ({} bytes)",
				HexDisplay::from(&hash),
				statement_len,
			);
			return MaybeInserted::Ignored
		}

		let mut evicted = HashSet::new();
		let mut would_free_size = 0;
		let priority = statement.priority().unwrap_or(0);
		let (max_size, max_count) = (validation.max_size as usize, validation.max_count as usize);
		// It may happen that we can't delete enough lower priority messages
		// to satisfy size constraints. We check for that before deleting anything,
		// taking into account channel message replacement.
		if let Some(account_rec) = self.accounts.get(account) {
			if let Some(channel) = statement.channel() {
				if let Some(channel_record) = account_rec.channels.get(&channel) {
					if priority <= channel_record.priority {
						// Trying to replace channel message with lower priority
						log::debug!(
							target: LOG_TARGET,
							"Ignored lower priority channel message: {:?} {} <= {}",
							HexDisplay::from(&hash),
							priority,
							channel_record.priority,
						);
						return MaybeInserted::Ignored
					} else {
						// Would replace channel message. Still need to check for size constraints
						// below.
						log::debug!(
							target: LOG_TARGET,
							"Replacing higher priority channel message: {:?} ({}) > {:?} ({})",
							HexDisplay::from(&hash),
							priority,
							HexDisplay::from(&channel_record.hash),
							channel_record.priority,
						);
						let key = PriorityKey {
							hash: channel_record.hash,
							priority: channel_record.priority,
						};
						if let Some((_channel, len)) = account_rec.by_priority.get(&key) {
							would_free_size = *len;
							evicted.insert(channel_record.hash);
						}
					}
				}
			}
			// Check if we can evict enough lower priority statements to satisfy constraints
			for (entry, (_, len)) in account_rec.by_priority.iter() {
				if (account_rec.data_size - would_free_size + statement_len <= max_size) &&
					account_rec.by_priority.len() + 1 - evicted.len() <= max_count
				{
					// Satisfied
					break
				}
				if evicted.contains(&entry.hash) {
					// Already accounted for above
					continue
				}
				if entry.priority >= priority {
					log::debug!(
						target: LOG_TARGET,
						"Ignored message due to constraints {:?} {} < {}",
						HexDisplay::from(&hash),
						priority,
						entry.priority,
					);
					return MaybeInserted::Ignored
				}
				evicted.insert(entry.hash);
				would_free_size += len;
			}
		}
		// Now check global constraints as well.
		for (entry, len) in self.by_global_priority.iter() {
			if (self.total_size - would_free_size + statement_len <= self.max_size) &&
				self.by_global_priority.len() + 1 - evicted.len() <= self.max_entries
			{
				// Satisfied
				break
			}
			if evicted.contains(&entry.hash) {
				// Already accounted for above
				continue
			}

			if entry.priority >= priority {
				log::debug!(
					target: LOG_TARGET,
					"Ignored message due global to constraints {:?} {} < {}",
					HexDisplay::from(&hash),
					priority,
					entry.priority,
				);
				return MaybeInserted::Ignored
			}
			evicted.insert(entry.hash);
			would_free_size += len;
		}

		for h in &evicted {
			self.make_expired(h, current_time);
		}
		self.insert_new(hash, *account, priority, statement);
		MaybeInserted::Inserted(evicted)
	}
}

impl Store {
	/// Create a new shared store instance. There should only be one per process.
	pub fn new_shared<Block, Client>(
		path: &std::path::Path,
		client: Arc<Client>,
		prometheus: Option<&PrometheusRegistry>,
	) -> Result<Arc<Store>>
	where
		Block: BlockT,
		Block::Hash: From<BlockHash>,
		Client: ProvideRuntimeApi<Block>
			+ HeaderBackend<Block>
			+ sc_client_api::ExecutorProvider<Block>
			+ Send
			+ Sync
			+ 'static,
		Client::Api: ValidateStatement<Block>,
	{
		let store = Arc::new(Self::new(path, client.clone(), prometheus)?);
		client.execution_extensions().register_statement_store(store.clone());
		Ok(store)
	}

	/// Create a new instance.
	fn new<Block, Client>(
		path: &std::path::Path,
		client: Arc<Client>,
		prometheus: Option<&PrometheusRegistry>,
	) -> Result<Store>
	where
		Block: BlockT,
		Block::Hash: From<BlockHash>,
		Client: ProvideRuntimeApi<Block> + HeaderBackend<Block> + Send + Sync + 'static,
		Client::Api: ValidateStatement<Block>,
	{
		let mut path: std::path::PathBuf = path.into();
		path.pop();
		path.push("statement");

		let mut config = parity_db::Options::with_columns(&path, col::COUNT);

		let mut statement_col = &mut config.columns[col::STATEMENTS as usize];
		statement_col.ref_counted = false;
		statement_col.preimage = true;
		statement_col.uniform = true;
		let db = parity_db::Db::open_or_create(&config).map_err(|e| Error::Db(e.to_string()))?;
		match db.get(col::META, &KEY_VERSION).map_err(|e| Error::Db(e.to_string()))? {
			Some(version) => {
				let version = u32::from_le_bytes(
					version
						.try_into()
						.map_err(|_| Error::Db("Error reading database version".into()))?,
				);
				if version != CURRENT_VERSION {
					return Err(Error::Db(format!("Unsupported database version: {version}")))
				}
			},
			None => {
				db.commit([(
					col::META,
					KEY_VERSION.to_vec(),
					Some(CURRENT_VERSION.to_le_bytes().to_vec()),
				)])
				.map_err(|e| Error::Db(e.to_string()))?;
			},
		}

		let validator = ClientWrapper { client, _block: Default::default() };
		let validate_fn = Box::new(move |block, source, statement| {
			validator.validate_statement(block, source, statement)
		});

		let store = Store {
			db,
			index: RwLock::new(Index::new()),
			validate_fn,
			time_override: None,
			metrics: PrometheusMetrics::new(prometheus),
		};
		store.populate()?;
		Ok(store)
	}

	fn populate(&self) -> Result<()> {
		{
			let mut index = self.index.write();
			self.db
				.iter_column_while(col::STATEMENTS, |item| {
					let statement = item.value;
					if let Ok(statement_with_meta) =
						StatementWithMeta::decode(&mut statement.as_slice())
					{
						let hash = statement_with_meta.statement.hash();
						log::trace!(
							target: LOG_TARGET,
							"Statement loaded {:?}",
							HexDisplay::from(&hash)
						);
						if let Some(account_id) = statement_with_meta.statement.account_id() {
							index.insert_new(
								hash,
								account_id,
								statement_with_meta.meta.global_priority,
								&statement_with_meta.statement,
							);
						}
					}
					true
				})
				.map_err(|e| Error::Db(e.to_string()))?;
			self.db
				.iter_column_while(col::EXPIRED, |item| {
					let expired_info = item.value;
					if let Ok((hash, timestamp)) =
						<(Hash, u64)>::decode(&mut expired_info.as_slice())
					{
						log::trace!(
							target: LOG_TARGET,
							"Statement loaded (expired): {:?}",
							HexDisplay::from(&hash)
						);
						index.insert_expired(hash, timestamp);
					}
					true
				})
				.map_err(|e| Error::Db(e.to_string()))?;
		}

		self.maintain();
		Ok(())
	}

	fn collect_statements<R>(
		&self,
		key: Option<DecryptionKey>,
		match_all_topics: &[Topic],
		mut f: impl FnMut(Statement) -> Option<R>,
	) -> Result<Vec<R>> {
		let mut result = Vec::new();
		let index = self.index.read();
		index.iter(key, match_all_topics, |hash| {
			match self.db.get(col::STATEMENTS, hash).map_err(|e| Error::Db(e.to_string()))? {
				Some(entry) => {
					if let Ok(statement) = StatementWithMeta::decode(&mut entry.as_slice()) {
						if let Some(data) = f(statement.statement) {
							result.push(data);
						}
					} else {
						// DB inconsistency
						log::warn!(
							target: LOG_TARGET,
							"Corrupt statement {:?}",
							HexDisplay::from(hash)
						);
					}
				},
				None => {
					// DB inconsistency
					log::warn!(
						target: LOG_TARGET,
						"Missing statement {:?}",
						HexDisplay::from(hash)
					);
				},
			}
			Ok(())
		})?;
		Ok(result)
	}

	/// Perform periodic store maintenance
	pub fn maintain(&self) {
		log::trace!(target: LOG_TARGET, "Started store maintenance");
		let deleted = self.index.write().maintain(self.timestamp());
		let deleted: Vec<_> =
			deleted.into_iter().map(|hash| (col::EXPIRED, hash.to_vec(), None)).collect();
		let count = deleted.len() as u64;
		if let Err(e) = self.db.commit(deleted) {
			log::warn!(target: LOG_TARGET, "Error writing to the statement database: {:?}", e);
		} else {
			self.metrics.report(|metrics| metrics.statements_pruned.inc_by(count));
		}
		log::trace!(
			target: LOG_TARGET,
			"Completed store maintenance. Purged: {}, Active: {}, Expired: {}",
			count,
			self.index.read().entries.len(),
			self.index.read().expired.len()
		);
	}

	fn timestamp(&self) -> u64 {
		self.time_override.unwrap_or_else(|| {
			std::time::SystemTime::now()
				.duration_since(std::time::UNIX_EPOCH)
				.unwrap_or_default()
				.as_secs()
		})
	}

	#[cfg(test)]
	fn set_time(&mut self, time: u64) {
		self.time_override = Some(time);
	}
}

impl StatementStore for Store {
	fn dump_encoded(&self) -> Result<Vec<(Hash, Vec<u8>)>> {
		let index = self.index.read();
		let mut result = Vec::with_capacity(index.entries.len());
		for h in self.index.read().entries.keys() {
			let encoded = self.db.get(col::STATEMENTS, h).map_err(|e| Error::Db(e.to_string()))?;
			if let Some(encoded) = encoded {
				if let Ok(entry) = StatementWithMeta::decode(&mut encoded.as_slice()) {
					entry.statement.using_encoded(|statement| {
						let hash = sp_statement_store::hash_encoded(statement);
						if !self.index.read().is_expired(&hash) {
							result.push((hash, entry.statement.encode()));
						}
					});
				}
			}
		}
		Ok(result)
	}

	/// Return all statements.
	fn dump(&self) -> Result<Vec<(Hash, Statement)>> {
		let index = self.index.read();
		let mut result = Vec::with_capacity(index.entries.len());
		for h in self.index.read().entries.keys() {
			let encoded = self.db.get(col::STATEMENTS, h).map_err(|e| Error::Db(e.to_string()))?;
			if let Some(encoded) = encoded {
				if let Ok(entry) = StatementWithMeta::decode(&mut encoded.as_slice()) {
					let hash = entry.statement.hash();
					result.push((hash, entry.statement));
				}
			}
		}
		Ok(result)
	}

	/// Returns a statement by hash.
	fn statement(&self, hash: &Hash) -> Result<Option<Statement>> {
		Ok(
			match self
				.db
				.get(col::STATEMENTS, hash.as_slice())
				.map_err(|e| Error::Db(e.to_string()))?
			{
				Some(entry) => {
					log::trace!(
						target: LOG_TARGET,
						"Queried statement {:?}",
						HexDisplay::from(hash)
					);
					Some(
						StatementWithMeta::decode(&mut entry.as_slice())
							.map_err(|e| Error::Decode(e.to_string()))?
							.statement,
					)
				},
				None => {
					log::trace!(
						target: LOG_TARGET,
						"Queried missing statement {:?}",
						HexDisplay::from(hash)
					);
					None
				},
			},
		)
	}

	/// Return the data of all known statements which include all topics and have no `DecryptionKey`
	/// field.
	fn broadcasts(&self, match_all_topics: &[Topic]) -> Result<Vec<Vec<u8>>> {
		self.collect_statements(None, match_all_topics, |statement| statement.into_data())
	}

	/// Return the data of all known statements whose decryption key is identified as `dest` (this
	/// will generally be the public key or a hash thereof for symmetric ciphers, or a hash of the
	/// private key for symmetric ciphers).
	fn posted(&self, match_all_topics: &[Topic], dest: [u8; 32]) -> Result<Vec<Vec<u8>>> {
		self.collect_statements(Some(dest), match_all_topics, |statement| statement.into_data())
	}

	/// Return the decrypted data of all known statements whose decryption key is identified as
	/// `dest`. The key must be available to the client.
	fn posted_clear(&self, match_all_topics: &[Topic], dest: [u8; 32]) -> Result<Vec<Vec<u8>>> {
		self.collect_statements(Some(dest), match_all_topics, |statement| statement.into_data())
	}

	/// Submit a statement to the store. Validates the statement and returns validation result.
	fn submit(&self, statement: Statement, source: StatementSource) -> SubmitResult {
		let hash = statement.hash();
		match self.index.read().query(&hash) {
			IndexQuery::Expired =>
				if !source.can_be_resubmitted() {
					return SubmitResult::KnownExpired
				},
			IndexQuery::Exists =>
				if !source.can_be_resubmitted() {
					return SubmitResult::Known
				},
			IndexQuery::Unknown => {},
		}

		let Some(account_id) = statement.account_id() else {
			log::debug!(
				target: LOG_TARGET,
				"Statement validation failed: Missing proof ({:?})",
				HexDisplay::from(&hash),
			);
			self.metrics.report(|metrics| metrics.validations_invalid.inc());
			return SubmitResult::Bad("No statement proof")
		};

		// Validate.
		let at_block = if let Some(Proof::OnChain { block_hash, .. }) = statement.proof() {
			Some(block_hash.clone())
		} else {
			None
		};
		let validation_result = (self.validate_fn)(at_block, source, statement.clone());
		let validation = match validation_result {
			Ok(validation) => validation,
			Err(InvalidStatement::BadProof) => {
				log::debug!(
					target: LOG_TARGET,
					"Statement validation failed: BadProof, {:?}",
					HexDisplay::from(&hash),
				);
				self.metrics.report(|metrics| metrics.validations_invalid.inc());
				return SubmitResult::Bad("Bad statement proof")
			},
			Err(InvalidStatement::NoProof) => {
				log::debug!(
					target: LOG_TARGET,
					"Statement validation failed: NoProof, {:?}",
					HexDisplay::from(&hash),
				);
				self.metrics.report(|metrics| metrics.validations_invalid.inc());
				return SubmitResult::Bad("Missing statement proof")
			},
			Err(InvalidStatement::InternalError) =>
				return SubmitResult::InternalError(Error::Runtime),
		};

		let statement_with_meta = StatementWithMeta {
			meta: StatementMeta { global_priority: validation.global_priority },
			statement,
		};

		let current_time = self.timestamp();
		let mut commit = Vec::new();
		{
			let mut index = self.index.write();

			let evicted = match index.insert(
				hash,
				&statement_with_meta.statement,
				&account_id,
				&validation,
				current_time,
			) {
				MaybeInserted::Ignored => return SubmitResult::Ignored,
				MaybeInserted::Inserted(evicted) => evicted,
			};

			commit.push((col::STATEMENTS, hash.to_vec(), Some(statement_with_meta.encode())));
			for hash in evicted {
				commit.push((col::STATEMENTS, hash.to_vec(), None));
				commit.push((col::EXPIRED, hash.to_vec(), Some((hash, current_time).encode())));
			}
			if let Err(e) = self.db.commit(commit) {
				log::debug!(
					target: LOG_TARGET,
					"Statement validation failed: database error {}, {:?}",
					e,
					statement_with_meta.statement
				);
				return SubmitResult::InternalError(Error::Db(e.to_string()))
			}
		} // Release index lock
		self.metrics.report(|metrics| metrics.submitted_statements.inc());
		let network_priority = if validation.global_priority > 0 {
			NetworkPriority::High
		} else {
			NetworkPriority::Low
		};
		log::trace!(target: LOG_TARGET, "Statement submitted: {:?}", HexDisplay::from(&hash));
		SubmitResult::New(network_priority)
	}

	/// Submit a SCALE-encoded statement.
	fn submit_encoded(&self, mut statement: &[u8], source: StatementSource) -> SubmitResult {
		match Statement::decode(&mut statement) {
			Ok(decoded) => self.submit(decoded, source),
			Err(e) => {
				log::debug!(
					target: LOG_TARGET,
					"Error decoding submitted statement. Failed with: {}",
					e
				);
				SubmitResult::Bad("Bad SCALE encoding")
			},
		}
	}

	fn remove(&self, hash: &Hash) -> Result<()> {
		let current_time = self.timestamp();
		{
			let mut index = self.index.write();
			if index.make_expired(hash, current_time) {
				let commit = [
					(col::STATEMENTS, hash.to_vec(), None),
					(col::EXPIRED, hash.to_vec(), Some((hash, current_time).encode())),
				];
				if let Err(e) = self.db.commit(commit) {
					log::debug!(
						target: LOG_TARGET,
						"Error removing statement: database error {}, {:?}",
						e,
						HexDisplay::from(hash),
					);
					return Err(Error::Db(e.to_string()))
				}
			}
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use crate::Store;
	use sp_core::Pair;
	use sp_statement_store::{
		runtime_api::{InvalidStatement, ValidStatement, ValidateStatement},
		AccountId, Channel, NetworkPriority, Proof, SignatureVerificationResult, Statement,
		StatementSource, StatementStore, SubmitResult, Topic,
	};

	type Extrinsic = sp_runtime::OpaqueExtrinsic;
	type Hash = sp_core::H256;
	type Hashing = sp_runtime::traits::BlakeTwo256;
	type BlockNumber = u64;
	type Header = sp_runtime::generic::Header<BlockNumber, Hashing>;
	type Block = sp_runtime::generic::Block<Header, Extrinsic>;

	const CORRECT_BLOCK_HASH: [u8; 32] = [1u8; 32];

	#[derive(Clone)]
	pub(crate) struct TestClient;

	pub(crate) struct RuntimeApi {
		_inner: TestClient,
	}

	impl sp_api::ProvideRuntimeApi<Block> for TestClient {
		type Api = RuntimeApi;
		fn runtime_api(&self) -> sp_api::ApiRef<Self::Api> {
			RuntimeApi { _inner: self.clone() }.into()
		}
	}
	sp_api::mock_impl_runtime_apis! {
		impl ValidateStatement<Block> for RuntimeApi {
			fn validate_statement(
				_source: StatementSource,
				statement: Statement,
			) -> std::result::Result<ValidStatement, InvalidStatement> {
				use crate::tests::account;
				match statement.verify_signature() {
					SignatureVerificationResult::Valid(_) => Ok(ValidStatement{global_priority: 10, max_count: 100, max_size: 1000}),
					SignatureVerificationResult::Invalid => Err(InvalidStatement::BadProof),
					SignatureVerificationResult::NoSignature => {
						if let Some(Proof::OnChain { block_hash, .. }) = statement.proof() {
							if block_hash == &CORRECT_BLOCK_HASH {
								let (global_priority, max_count, max_size) = match statement.account_id() {
									Some(a) if a == account(1) => (10, 1, 1000),
									Some(a) if a == account(2) => (20, 2, 1000),
									Some(a) if a == account(3) => (30, 3, 1000),
									Some(a) if a == account(4) => (40, 4, 1000),
									_ => (0, 2, 2000),
								};
								Ok(ValidStatement{ global_priority, max_count, max_size })
							} else {
								Err(InvalidStatement::BadProof)
							}
						} else {
							Err(InvalidStatement::BadProof)
						}
					}
				}
			}
		}
	}

	impl sp_blockchain::HeaderBackend<Block> for TestClient {
		fn header(&self, _hash: Hash) -> sp_blockchain::Result<Option<Header>> {
			unimplemented!()
		}
		fn info(&self) -> sp_blockchain::Info<Block> {
			sp_blockchain::Info {
				best_hash: CORRECT_BLOCK_HASH.into(),
				best_number: 0,
				genesis_hash: Default::default(),
				finalized_hash: CORRECT_BLOCK_HASH.into(),
				finalized_number: 1,
				finalized_state: None,
				number_leaves: 0,
				block_gap: None,
			}
		}
		fn status(&self, _hash: Hash) -> sp_blockchain::Result<sp_blockchain::BlockStatus> {
			unimplemented!()
		}
		fn number(&self, _hash: Hash) -> sp_blockchain::Result<Option<BlockNumber>> {
			unimplemented!()
		}
		fn hash(&self, _number: BlockNumber) -> sp_blockchain::Result<Option<Hash>> {
			unimplemented!()
		}
	}

	fn test_store() -> (Store, tempfile::TempDir) {
		let _ = env_logger::try_init();
		let temp_dir = tempfile::Builder::new().tempdir().expect("Error creating test dir");

		let client = std::sync::Arc::new(TestClient);
		let mut path: std::path::PathBuf = temp_dir.path().into();
		path.push("db");
		let store = Store::new(&path, client, None).unwrap();
		(store, temp_dir) // return order is important. Store must be dropped before TempDir
	}

	fn signed_statement(data: u8) -> Statement {
		signed_statement_with_topics(data, &[])
	}

	fn signed_statement_with_topics(data: u8, topics: &[Topic]) -> Statement {
		let mut statement = Statement::new();
		statement.set_plain_data(vec![data]);
		for i in 0..topics.len() {
			statement.set_topic(i, topics[i].clone());
		}
		let kp = sp_core::ed25519::Pair::from_string("//Alice", None).unwrap();
		statement.sign_ed25519_private(&kp);
		statement
	}

	fn topic(data: u64) -> Topic {
		let mut topic: Topic = Default::default();
		topic[0..8].copy_from_slice(&data.to_le_bytes());
		topic
	}

	fn account(id: u64) -> AccountId {
		let mut account: AccountId = Default::default();
		account[0..8].copy_from_slice(&id.to_le_bytes());
		account
	}

	fn channel(id: u64) -> Channel {
		let mut channel: Channel = Default::default();
		channel[0..8].copy_from_slice(&id.to_le_bytes());
		channel
	}

	fn statement(account_id: u64, priority: u32, c: Option<u64>, data_len: usize) -> Statement {
		let mut statement = Statement::new();
		let mut data = Vec::new();
		data.resize(data_len, 0);
		statement.set_plain_data(data);
		statement.set_priority(priority);
		if let Some(c) = c {
			statement.set_channel(channel(c));
		}
		statement.set_proof(Proof::OnChain {
			block_hash: CORRECT_BLOCK_HASH,
			who: account(account_id),
			event_index: 0,
		});
		statement
	}

	#[test]
	fn submit_one() {
		let (store, _temp) = test_store();
		let statement0 = signed_statement(0);
		assert_eq!(
			store.submit(statement0, StatementSource::Network),
			SubmitResult::New(NetworkPriority::High)
		);
		let unsigned = statement(0, 1, None, 0);
		assert_eq!(
			store.submit(unsigned, StatementSource::Network),
			SubmitResult::New(NetworkPriority::Low)
		);
	}

	#[test]
	fn save_and_load_statements() {
		let (store, temp) = test_store();
		let statement0 = signed_statement(0);
		let statement1 = signed_statement(1);
		let statement2 = signed_statement(2);
		assert_eq!(
			store.submit(statement0.clone(), StatementSource::Network),
			SubmitResult::New(NetworkPriority::High)
		);
		assert_eq!(
			store.submit(statement1.clone(), StatementSource::Network),
			SubmitResult::New(NetworkPriority::High)
		);
		assert_eq!(
			store.submit(statement2.clone(), StatementSource::Network),
			SubmitResult::New(NetworkPriority::High)
		);
		assert_eq!(store.dump().unwrap().len(), 3);
		assert_eq!(store.broadcasts(&[]).unwrap().len(), 3);
		assert_eq!(store.statement(&statement1.hash()).unwrap(), Some(statement1.clone()));
		drop(store);

		let client = std::sync::Arc::new(TestClient);
		let mut path: std::path::PathBuf = temp.path().into();
		path.push("db");
		let store = Store::new(&path, client, None).unwrap();
		assert_eq!(store.dump().unwrap().len(), 3);
		assert_eq!(store.broadcasts(&[]).unwrap().len(), 3);
		assert_eq!(store.statement(&statement1.hash()).unwrap(), Some(statement1));
	}

	#[test]
	fn search_by_topic() {
		let (store, _temp) = test_store();
		let statement0 = signed_statement(0);
		let statement1 = signed_statement_with_topics(1, &[topic(0)]);
		let statement2 = signed_statement_with_topics(2, &[topic(0), topic(1)]);
		let statement3 = signed_statement_with_topics(3, &[topic(0), topic(1), topic(2)]);
		let statement4 =
			signed_statement_with_topics(4, &[topic(0), topic(42), topic(2), topic(3)]);
		let statements = vec![statement0, statement1, statement2, statement3, statement4];
		for s in &statements {
			store.submit(s.clone(), StatementSource::Network);
		}

		let assert_topics = |topics: &[u64], expected: &[u8]| {
			let topics: Vec<_> = topics.iter().map(|t| topic(*t)).collect();
			let mut got_vals: Vec<_> =
				store.broadcasts(&topics).unwrap().into_iter().map(|d| d[0]).collect();
			got_vals.sort();
			assert_eq!(expected.to_vec(), got_vals);
		};

		assert_topics(&[], &[0, 1, 2, 3, 4]);
		assert_topics(&[0], &[1, 2, 3, 4]);
		assert_topics(&[1], &[2, 3]);
		assert_topics(&[2], &[3, 4]);
		assert_topics(&[3], &[4]);
		assert_topics(&[42], &[4]);

		assert_topics(&[0, 1], &[2, 3]);
		assert_topics(&[1, 2], &[3]);
		assert_topics(&[99], &[]);
		assert_topics(&[0, 99], &[]);
		assert_topics(&[0, 1, 2, 3, 42], &[]);
	}

	#[test]
	fn constraints() {
		let (store, _temp) = test_store();

		store.index.write().max_size = 3000;
		let source = StatementSource::Network;
		let ok = SubmitResult::New(NetworkPriority::High);
		let ignored = SubmitResult::Ignored;

		// Account 1 (limit = 1 msg, 1000 bytes)

		// Oversized statement is not allowed. Limit for account 1 is 1 msg, 1000 bytes
		assert_eq!(store.submit(statement(1, 1, Some(1), 2000), source), ignored);
		assert_eq!(store.submit(statement(1, 1, Some(1), 500), source), ok);
		// Would not replace channel message with same priority
		assert_eq!(store.submit(statement(1, 1, Some(1), 200), source), ignored);
		assert_eq!(store.submit(statement(1, 2, Some(1), 600), source), ok);
		// Submit another message to another channel with lower priority. Should not be allowed
		// because msg count limit is 1
		assert_eq!(store.submit(statement(1, 1, Some(2), 100), source), ignored);
		assert_eq!(store.index.read().expired.len(), 1);

		// Account 2 (limit = 2 msg, 1000 bytes)

		assert_eq!(store.submit(statement(2, 1, None, 500), source), ok);
		assert_eq!(store.submit(statement(2, 2, None, 100), source), ok);
		// Should evict priority 1
		assert_eq!(store.submit(statement(2, 3, None, 500), source), ok);
		assert_eq!(store.index.read().expired.len(), 2);
		// Should evict all
		assert_eq!(store.submit(statement(2, 4, None, 1000), source), ok);
		assert_eq!(store.index.read().expired.len(), 4);

		// Account 3 (limit = 3 msg, 1000 bytes)

		assert_eq!(store.submit(statement(3, 2, Some(1), 300), source), ok);
		assert_eq!(store.submit(statement(3, 3, Some(2), 300), source), ok);
		assert_eq!(store.submit(statement(3, 4, Some(3), 300), source), ok);
		// Should evict 2 and 3
		assert_eq!(store.submit(statement(3, 5, None, 500), source), ok);
		assert_eq!(store.index.read().expired.len(), 6);

		assert_eq!(store.index.read().total_size, 2400);
		assert_eq!(store.index.read().entries.len(), 4);

		// Should be over the global size limit
		assert_eq!(store.submit(statement(4, 1, None, 700), source), ignored);
		// Should be over the global count limit
		store.index.write().max_entries = 4;
		assert_eq!(store.submit(statement(4, 1, None, 100), source), ignored);
		// Should evict statement from account 1
		assert_eq!(store.submit(statement(4, 6, None, 100), source), ok);
		assert_eq!(store.index.read().expired.len(), 7);

		let mut expected_statements = vec![
			statement(2, 4, None, 1000).hash(),
			statement(3, 4, Some(3), 300).hash(),
			statement(3, 5, None, 500).hash(),
			statement(4, 6, None, 100).hash(),
		];
		expected_statements.sort();
		let mut statements: Vec<_> =
			store.dump().unwrap().into_iter().map(|(hash, _)| hash).collect();
		statements.sort();
		assert_eq!(expected_statements, statements);
	}

	#[test]
	fn expired_statements_are_purged() {
		use super::PURGE_AFTER;
		let (mut store, temp) = test_store();
		let mut statement = statement(1, 1, Some(3), 100);
		store.set_time(0);
		statement.set_topic(0, topic(4));
		store.submit(statement.clone(), StatementSource::Network);
		assert_eq!(store.index.read().entries.len(), 1);
		store.remove(&statement.hash()).unwrap();
		assert_eq!(store.index.read().entries.len(), 0);
		assert_eq!(store.index.read().by_global_priority.len(), 0);
		assert_eq!(store.index.read().accounts.len(), 0);
		store.set_time(PURGE_AFTER + 1);
		store.maintain();
		assert_eq!(store.index.read().expired.len(), 0);
		drop(store);

		let client = std::sync::Arc::new(TestClient);
		let mut path: std::path::PathBuf = temp.path().into();
		path.push("db");
		let store = Store::new(&path, client, None).unwrap();
		assert_eq!(store.dump().unwrap().len(), 0);
		assert_eq!(store.index.read().expired.len(), 0);
	}
}
