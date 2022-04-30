// This file is part of Substrate.

// Copyright (C) 2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_alliance
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-10-11, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/substrate
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_alliance
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/alliance/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_alliance.
pub trait WeightInfo {
	fn propose_proposed(b: u32, x: u32, y: u32, p: u32, ) -> Weight;
	fn vote(x: u32, y: u32, ) -> Weight;
	fn veto(p: u32, ) -> Weight;
	fn close_early_disapproved(x: u32, y: u32, p: u32, ) -> Weight;
	fn close_early_approved(b: u32, x: u32, y: u32, p: u32, ) -> Weight;
	fn close_disapproved(x: u32, y: u32, p: u32, ) -> Weight;
	fn close_approved(b: u32, x: u32, y: u32, p: u32, ) -> Weight;
	fn init_members(x: u32, y: u32, z: u32, ) -> Weight;
	fn set_rule() -> Weight;
	fn announce() -> Weight;
	fn remove_announcement() -> Weight;
	fn join_alliance() -> Weight;
	fn nominate_ally() -> Weight;
	fn elevate_ally() -> Weight;
	fn retire() -> Weight;
	fn kick_member() -> Weight;
	fn add_blacklist_items(n: u32, l: u32, ) -> Weight;
	fn remove_blacklist_items(n: u32, l: u32, ) -> Weight;
}

/// Weights for pallet_alliance using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion ProposalCount (r:1 w:1)
	// Storage: AllianceMotion Voting (r:0 w:1)
	fn propose_proposed(_b: u32, _x: u32, y: u32, p: u32, ) -> Weight {
		(39_992_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((44_000 as Weight).saturating_mul(y as Weight))
			// Standard Error: 2_000
			.saturating_add((323_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Alliance Members (r:2 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	fn vote(x: u32, y: u32, ) -> Weight {
		(36_649_000 as Weight)
			// Standard Error: 90_000
			.saturating_add((42_000 as Weight).saturating_mul(x as Weight))
			// Standard Error: 3_000
			.saturating_add((195_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion Voting (r:0 w:1)
	fn veto(p: u32, ) -> Weight {
		(30_301_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((330_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	fn close_early_disapproved(x: u32, y: u32, p: u32, ) -> Weight {
		(40_472_000 as Weight)
			// Standard Error: 69_000
			.saturating_add((485_000 as Weight).saturating_mul(x as Weight))
			// Standard Error: 2_000
			.saturating_add((192_000 as Weight).saturating_mul(y as Weight))
			// Standard Error: 2_000
			.saturating_add((330_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	fn close_early_approved(b: u32, x: u32, y: u32, p: u32, ) -> Weight {
		(52_076_000 as Weight)
			// Standard Error: 0
			.saturating_add((4_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 77_000
			.saturating_add((194_000 as Weight).saturating_mul(x as Weight))
			// Standard Error: 3_000
			.saturating_add((188_000 as Weight).saturating_mul(y as Weight))
			// Standard Error: 2_000
			.saturating_add((329_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Prime (r:1 w:0)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	fn close_disapproved(x: u32, y: u32, p: u32, ) -> Weight {
		(47_009_000 as Weight)
			// Standard Error: 66_000
			.saturating_add((256_000 as Weight).saturating_mul(x as Weight))
			// Standard Error: 2_000
			.saturating_add((176_000 as Weight).saturating_mul(y as Weight))
			// Standard Error: 2_000
			.saturating_add((327_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Prime (r:1 w:0)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	fn close_approved(b: u32, x: u32, y: u32, p: u32, ) -> Weight {
		(43_650_000 as Weight)
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 85_000
			.saturating_add((124_000 as Weight).saturating_mul(x as Weight))
			// Standard Error: 3_000
			.saturating_add((199_000 as Weight).saturating_mul(y as Weight))
			// Standard Error: 3_000
			.saturating_add((326_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Alliance Members (r:3 w:3)
	// Storage: AllianceMotion Members (r:1 w:1)
	fn init_members(_x: u32, y: u32, z: u32, ) -> Weight {
		(45_100_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((162_000 as Weight).saturating_mul(y as Weight))
			// Standard Error: 4_000
			.saturating_add((151_000 as Weight).saturating_mul(z as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Alliance Rule (r:0 w:1)
	fn set_rule() -> Weight {
		(14_517_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Alliance Announcements (r:1 w:1)
	fn announce() -> Weight {
		(16_801_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Alliance Announcements (r:1 w:1)
	fn remove_announcement() -> Weight {
		(17_133_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Alliance AccountBlacklist (r:1 w:0)
	// Storage: Alliance Members (r:4 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Alliance DepositOf (r:0 w:1)
	fn join_alliance() -> Weight {
		(95_370_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	// Storage: Alliance Members (r:4 w:0)
	// Storage: Alliance AccountBlacklist (r:1 w:0)
	fn nominate_ally() -> Weight {
		(44_764_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(6 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: Alliance Members (r:3 w:2)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	fn elevate_ally() -> Weight {
		(44_013_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: Alliance KickingMembers (r:1 w:0)
	// Storage: Alliance Members (r:3 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: Alliance DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	fn retire() -> Weight {
		(60_183_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Alliance KickingMembers (r:1 w:0)
	// Storage: Alliance Members (r:3 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: Alliance DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	fn kick_member() -> Weight {
		(67_467_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(5 as Weight))
	}
	// Storage: Alliance AccountBlacklist (r:1 w:1)
	// Storage: Alliance WebsiteBlacklist (r:1 w:1)
	fn add_blacklist_items(n: u32, l: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 16_000
			.saturating_add((2_673_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 7_000
			.saturating_add((224_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: Alliance AccountBlacklist (r:1 w:1)
	// Storage: Alliance WebsiteBlacklist (r:1 w:1)
	fn remove_blacklist_items(n: u32, l: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 343_000
			.saturating_add((59_025_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 153_000
			.saturating_add((6_725_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion ProposalCount (r:1 w:1)
	// Storage: AllianceMotion Voting (r:0 w:1)
	fn propose_proposed(_b: u32, _x: u32, y: u32, p: u32, ) -> Weight {
		(39_992_000 as Weight)
			// Standard Error: 2_000
			.saturating_add((44_000 as Weight).saturating_mul(y as Weight))
			// Standard Error: 2_000
			.saturating_add((323_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Alliance Members (r:2 w:0)
	// Storage: AllianceMotion Voting (r:1 w:1)
	fn vote(x: u32, y: u32, ) -> Weight {
		(36_649_000 as Weight)
			// Standard Error: 90_000
			.saturating_add((42_000 as Weight).saturating_mul(x as Weight))
			// Standard Error: 3_000
			.saturating_add((195_000 as Weight).saturating_mul(y as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	// Storage: AllianceMotion Voting (r:0 w:1)
	fn veto(p: u32, ) -> Weight {
		(30_301_000 as Weight)
			// Standard Error: 1_000
			.saturating_add((330_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(3 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	fn close_early_disapproved(x: u32, y: u32, p: u32, ) -> Weight {
		(40_472_000 as Weight)
			// Standard Error: 69_000
			.saturating_add((485_000 as Weight).saturating_mul(x as Weight))
			// Standard Error: 2_000
			.saturating_add((192_000 as Weight).saturating_mul(y as Weight))
			// Standard Error: 2_000
			.saturating_add((330_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	fn close_early_approved(b: u32, x: u32, y: u32, p: u32, ) -> Weight {
		(52_076_000 as Weight)
			// Standard Error: 0
			.saturating_add((4_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 77_000
			.saturating_add((194_000 as Weight).saturating_mul(x as Weight))
			// Standard Error: 3_000
			.saturating_add((188_000 as Weight).saturating_mul(y as Weight))
			// Standard Error: 2_000
			.saturating_add((329_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(5 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Prime (r:1 w:0)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	fn close_disapproved(x: u32, y: u32, p: u32, ) -> Weight {
		(47_009_000 as Weight)
			// Standard Error: 66_000
			.saturating_add((256_000 as Weight).saturating_mul(x as Weight))
			// Standard Error: 2_000
			.saturating_add((176_000 as Weight).saturating_mul(y as Weight))
			// Standard Error: 2_000
			.saturating_add((327_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Alliance Members (r:1 w:0)
	// Storage: AllianceMotion ProposalOf (r:1 w:1)
	// Storage: AllianceMotion Voting (r:1 w:1)
	// Storage: AllianceMotion Members (r:1 w:0)
	// Storage: AllianceMotion Prime (r:1 w:0)
	// Storage: AllianceMotion Proposals (r:1 w:1)
	fn close_approved(b: u32, x: u32, y: u32, p: u32, ) -> Weight {
		(43_650_000 as Weight)
			// Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(b as Weight))
			// Standard Error: 85_000
			.saturating_add((124_000 as Weight).saturating_mul(x as Weight))
			// Standard Error: 3_000
			.saturating_add((199_000 as Weight).saturating_mul(y as Weight))
			// Standard Error: 3_000
			.saturating_add((326_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Alliance Members (r:3 w:3)
	// Storage: AllianceMotion Members (r:1 w:1)
	fn init_members(_x: u32, y: u32, z: u32, ) -> Weight {
		(45_100_000 as Weight)
			// Standard Error: 4_000
			.saturating_add((162_000 as Weight).saturating_mul(y as Weight))
			// Standard Error: 4_000
			.saturating_add((151_000 as Weight).saturating_mul(z as Weight))
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Alliance Rule (r:0 w:1)
	fn set_rule() -> Weight {
		(14_517_000 as Weight)
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Alliance Announcements (r:1 w:1)
	fn announce() -> Weight {
		(16_801_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Alliance Announcements (r:1 w:1)
	fn remove_announcement() -> Weight {
		(17_133_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(1 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Alliance AccountBlacklist (r:1 w:0)
	// Storage: Alliance Members (r:4 w:0)
	// Storage: System Account (r:1 w:1)
	// Storage: Alliance DepositOf (r:0 w:1)
	fn join_alliance() -> Weight {
		(95_370_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(3 as Weight))
	}
	// Storage: Alliance Members (r:4 w:0)
	// Storage: Alliance AccountBlacklist (r:1 w:0)
	fn nominate_ally() -> Weight {
		(44_764_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(6 as Weight))
			.saturating_add(RocksDbWeight::get().writes(1 as Weight))
	}
	// Storage: Alliance Members (r:3 w:2)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	fn elevate_ally() -> Weight {
		(44_013_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(4 as Weight))
			.saturating_add(RocksDbWeight::get().writes(4 as Weight))
	}
	// Storage: Alliance KickingMembers (r:1 w:0)
	// Storage: Alliance Members (r:3 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: Alliance DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	fn retire() -> Weight {
		(60_183_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: Alliance KickingMembers (r:1 w:0)
	// Storage: Alliance Members (r:3 w:1)
	// Storage: AllianceMotion Proposals (r:1 w:0)
	// Storage: Alliance DepositOf (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: AllianceMotion Members (r:0 w:1)
	// Storage: AllianceMotion Prime (r:0 w:1)
	fn kick_member() -> Weight {
		(67_467_000 as Weight)
			.saturating_add(RocksDbWeight::get().reads(7 as Weight))
			.saturating_add(RocksDbWeight::get().writes(5 as Weight))
	}
	// Storage: Alliance AccountBlacklist (r:1 w:1)
	// Storage: Alliance WebsiteBlacklist (r:1 w:1)
	fn add_blacklist_items(n: u32, l: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 16_000
			.saturating_add((2_673_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 7_000
			.saturating_add((224_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
	// Storage: Alliance AccountBlacklist (r:1 w:1)
	// Storage: Alliance WebsiteBlacklist (r:1 w:1)
	fn remove_blacklist_items(n: u32, l: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 343_000
			.saturating_add((59_025_000 as Weight).saturating_mul(n as Weight))
			// Standard Error: 153_000
			.saturating_add((6_725_000 as Weight).saturating_mul(l as Weight))
			.saturating_add(RocksDbWeight::get().reads(2 as Weight))
			.saturating_add(RocksDbWeight::get().writes(2 as Weight))
	}
}
