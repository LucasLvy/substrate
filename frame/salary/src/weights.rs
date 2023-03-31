// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_salary
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-03-31, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-hbsh75as-project-145-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_salary
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/salary/src/weights.rs
// --header=./HEADER-APACHE2
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_salary.
pub trait WeightInfo {
	fn init() -> Weight;
	fn bump() -> Weight;
	fn induct() -> Weight;
	fn register() -> Weight;
	fn payout() -> Weight;
	fn payout_other() -> Weight;
	fn check_payment() -> Weight;
}

/// Weights for pallet_salary using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Salary Status (r:1 w:1)
	/// Proof: Salary Status (max_values: Some(1), max_size: Some(56), added: 551, mode: MaxEncodedLen)
	fn init() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `1541`
		// Minimum execution time: 11_685_000 picoseconds.
		Weight::from_parts(12_179_000, 1541)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Salary Status (r:1 w:1)
	/// Proof: Salary Status (max_values: Some(1), max_size: Some(56), added: 551, mode: MaxEncodedLen)
	fn bump() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `86`
		//  Estimated: `1541`
		// Minimum execution time: 12_954_000 picoseconds.
		Weight::from_parts(13_734_000, 1541)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Salary Status (r:1 w:0)
	/// Proof: Salary Status (max_values: Some(1), max_size: Some(56), added: 551, mode: MaxEncodedLen)
	/// Storage: RankedCollective Members (r:1 w:0)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: Salary Claimant (r:1 w:1)
	/// Proof: Salary Claimant (max_values: None, max_size: Some(78), added: 2553, mode: MaxEncodedLen)
	fn induct() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `362`
		//  Estimated: `8591`
		// Minimum execution time: 19_996_000 picoseconds.
		Weight::from_parts(20_505_000, 8591)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: RankedCollective Members (r:1 w:0)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: Salary Status (r:1 w:1)
	/// Proof: Salary Status (max_values: Some(1), max_size: Some(56), added: 551, mode: MaxEncodedLen)
	/// Storage: Salary Claimant (r:1 w:1)
	/// Proof: Salary Claimant (max_values: None, max_size: Some(78), added: 2553, mode: MaxEncodedLen)
	fn register() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `429`
		//  Estimated: `8591`
		// Minimum execution time: 24_420_000 picoseconds.
		Weight::from_parts(25_226_000, 8591)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Salary Status (r:1 w:1)
	/// Proof: Salary Status (max_values: Some(1), max_size: Some(56), added: 551, mode: MaxEncodedLen)
	/// Storage: Salary Claimant (r:1 w:1)
	/// Proof: Salary Claimant (max_values: None, max_size: Some(78), added: 2553, mode: MaxEncodedLen)
	/// Storage: RankedCollective Members (r:1 w:0)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	fn payout() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `429`
		//  Estimated: `8591`
		// Minimum execution time: 67_698_000 picoseconds.
		Weight::from_parts(69_980_000, 8591)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Salary Status (r:1 w:1)
	/// Proof: Salary Status (max_values: Some(1), max_size: Some(56), added: 551, mode: MaxEncodedLen)
	/// Storage: Salary Claimant (r:1 w:1)
	/// Proof: Salary Claimant (max_values: None, max_size: Some(78), added: 2553, mode: MaxEncodedLen)
	/// Storage: RankedCollective Members (r:1 w:0)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn payout_other() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `429`
		//  Estimated: `12184`
		// Minimum execution time: 69_662_000 picoseconds.
		Weight::from_parts(71_748_000, 12184)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: Salary Status (r:1 w:1)
	/// Proof: Salary Status (max_values: Some(1), max_size: Some(56), added: 551, mode: MaxEncodedLen)
	/// Storage: Salary Claimant (r:1 w:1)
	/// Proof: Salary Claimant (max_values: None, max_size: Some(78), added: 2553, mode: MaxEncodedLen)
	fn check_payment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `170`
		//  Estimated: `5084`
		// Minimum execution time: 12_941_000 picoseconds.
		Weight::from_parts(13_248_000, 5084)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Salary Status (r:1 w:1)
	/// Proof: Salary Status (max_values: Some(1), max_size: Some(56), added: 551, mode: MaxEncodedLen)
	fn init() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4`
		//  Estimated: `1541`
		// Minimum execution time: 11_685_000 picoseconds.
		Weight::from_parts(12_179_000, 1541)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Salary Status (r:1 w:1)
	/// Proof: Salary Status (max_values: Some(1), max_size: Some(56), added: 551, mode: MaxEncodedLen)
	fn bump() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `86`
		//  Estimated: `1541`
		// Minimum execution time: 12_954_000 picoseconds.
		Weight::from_parts(13_734_000, 1541)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Salary Status (r:1 w:0)
	/// Proof: Salary Status (max_values: Some(1), max_size: Some(56), added: 551, mode: MaxEncodedLen)
	/// Storage: RankedCollective Members (r:1 w:0)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: Salary Claimant (r:1 w:1)
	/// Proof: Salary Claimant (max_values: None, max_size: Some(78), added: 2553, mode: MaxEncodedLen)
	fn induct() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `362`
		//  Estimated: `8591`
		// Minimum execution time: 19_996_000 picoseconds.
		Weight::from_parts(20_505_000, 8591)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: RankedCollective Members (r:1 w:0)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: Salary Status (r:1 w:1)
	/// Proof: Salary Status (max_values: Some(1), max_size: Some(56), added: 551, mode: MaxEncodedLen)
	/// Storage: Salary Claimant (r:1 w:1)
	/// Proof: Salary Claimant (max_values: None, max_size: Some(78), added: 2553, mode: MaxEncodedLen)
	fn register() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `429`
		//  Estimated: `8591`
		// Minimum execution time: 24_420_000 picoseconds.
		Weight::from_parts(25_226_000, 8591)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Salary Status (r:1 w:1)
	/// Proof: Salary Status (max_values: Some(1), max_size: Some(56), added: 551, mode: MaxEncodedLen)
	/// Storage: Salary Claimant (r:1 w:1)
	/// Proof: Salary Claimant (max_values: None, max_size: Some(78), added: 2553, mode: MaxEncodedLen)
	/// Storage: RankedCollective Members (r:1 w:0)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	fn payout() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `429`
		//  Estimated: `8591`
		// Minimum execution time: 67_698_000 picoseconds.
		Weight::from_parts(69_980_000, 8591)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Salary Status (r:1 w:1)
	/// Proof: Salary Status (max_values: Some(1), max_size: Some(56), added: 551, mode: MaxEncodedLen)
	/// Storage: Salary Claimant (r:1 w:1)
	/// Proof: Salary Claimant (max_values: None, max_size: Some(78), added: 2553, mode: MaxEncodedLen)
	/// Storage: RankedCollective Members (r:1 w:0)
	/// Proof: RankedCollective Members (max_values: None, max_size: Some(42), added: 2517, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn payout_other() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `429`
		//  Estimated: `12184`
		// Minimum execution time: 69_662_000 picoseconds.
		Weight::from_parts(71_748_000, 12184)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: Salary Status (r:1 w:1)
	/// Proof: Salary Status (max_values: Some(1), max_size: Some(56), added: 551, mode: MaxEncodedLen)
	/// Storage: Salary Claimant (r:1 w:1)
	/// Proof: Salary Claimant (max_values: None, max_size: Some(78), added: 2553, mode: MaxEncodedLen)
	fn check_payment() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `170`
		//  Estimated: `5084`
		// Minimum execution time: 12_941_000 picoseconds.
		Weight::from_parts(13_248_000, 5084)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
}
