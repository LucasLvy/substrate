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

//! Autogenerated weights for pallet_recovery
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
// --pallet=pallet_recovery
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/recovery/src/weights.rs
// --header=./HEADER-APACHE2
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_recovery.
pub trait WeightInfo {
	fn as_recovered() -> Weight;
	fn set_recovered() -> Weight;
	fn create_recovery(n: u32, ) -> Weight;
	fn initiate_recovery() -> Weight;
	fn vouch_recovery(n: u32, ) -> Weight;
	fn claim_recovery(n: u32, ) -> Weight;
	fn close_recovery(n: u32, ) -> Weight;
	fn remove_recovery(n: u32, ) -> Weight;
	fn cancel_recovered() -> Weight;
}

/// Weights for pallet_recovery using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: Recovery Proxy (r:1 w:0)
	/// Proof: Recovery Proxy (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	fn as_recovered() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `281`
		//  Estimated: `3545`
		// Minimum execution time: 10_466_000 picoseconds.
		Weight::from_parts(10_670_000, 3545)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: Recovery Proxy (r:0 w:1)
	/// Proof: Recovery Proxy (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	fn set_recovered() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 10_297_000 picoseconds.
		Weight::from_parts(10_840_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Recovery Recoverable (r:1 w:1)
	/// Proof: Recovery Recoverable (max_values: None, max_size: Some(351), added: 2826, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn create_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `175`
		//  Estimated: `3816`
		// Minimum execution time: 28_510_000 picoseconds.
		Weight::from_parts(30_060_981, 3816)
			// Standard Error: 7_424
			.saturating_add(Weight::from_parts(56_594, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Recovery Recoverable (r:1 w:0)
	/// Proof: Recovery Recoverable (max_values: None, max_size: Some(351), added: 2826, mode: MaxEncodedLen)
	/// Storage: Recovery ActiveRecoveries (r:1 w:1)
	/// Proof: Recovery ActiveRecoveries (max_values: None, max_size: Some(389), added: 2864, mode: MaxEncodedLen)
	fn initiate_recovery() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `272`
		//  Estimated: `7670`
		// Minimum execution time: 31_478_000 picoseconds.
		Weight::from_parts(32_960_000, 7670)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Recovery Recoverable (r:1 w:0)
	/// Proof: Recovery Recoverable (max_values: None, max_size: Some(351), added: 2826, mode: MaxEncodedLen)
	/// Storage: Recovery ActiveRecoveries (r:1 w:1)
	/// Proof: Recovery ActiveRecoveries (max_values: None, max_size: Some(389), added: 2864, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn vouch_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `360 + n * (64 ±0)`
		//  Estimated: `7670`
		// Minimum execution time: 20_501_000 picoseconds.
		Weight::from_parts(21_889_392, 7670)
			// Standard Error: 6_732
			.saturating_add(Weight::from_parts(206_317, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Recovery Recoverable (r:1 w:0)
	/// Proof: Recovery Recoverable (max_values: None, max_size: Some(351), added: 2826, mode: MaxEncodedLen)
	/// Storage: Recovery ActiveRecoveries (r:1 w:0)
	/// Proof: Recovery ActiveRecoveries (max_values: None, max_size: Some(389), added: 2864, mode: MaxEncodedLen)
	/// Storage: Recovery Proxy (r:1 w:1)
	/// Proof: Recovery Proxy (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn claim_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `392 + n * (64 ±0)`
		//  Estimated: `11215`
		// Minimum execution time: 25_601_000 picoseconds.
		Weight::from_parts(27_004_521, 11215)
			// Standard Error: 7_284
			.saturating_add(Weight::from_parts(109_212, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Recovery ActiveRecoveries (r:1 w:1)
	/// Proof: Recovery ActiveRecoveries (max_values: None, max_size: Some(389), added: 2864, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn close_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `513 + n * (32 ±0)`
		//  Estimated: `7447`
		// Minimum execution time: 35_970_000 picoseconds.
		Weight::from_parts(37_632_353, 7447)
			// Standard Error: 8_484
			.saturating_add(Weight::from_parts(213_623, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: Recovery ActiveRecoveries (r:1 w:0)
	/// Proof: Recovery ActiveRecoveries (max_values: None, max_size: Some(389), added: 2864, mode: MaxEncodedLen)
	/// Storage: Recovery Recoverable (r:1 w:1)
	/// Proof: Recovery Recoverable (max_values: None, max_size: Some(351), added: 2826, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn remove_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `270 + n * (32 ±0)`
		//  Estimated: `7670`
		// Minimum execution time: 33_812_000 picoseconds.
		Weight::from_parts(35_604_811, 7670)
			// Standard Error: 10_784
			.saturating_add(Weight::from_parts(70_892, 0).saturating_mul(n.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: Recovery Proxy (r:1 w:1)
	/// Proof: Recovery Proxy (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	fn cancel_recovered() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `281`
		//  Estimated: `3545`
		// Minimum execution time: 12_545_000 picoseconds.
		Weight::from_parts(13_092_000, 3545)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: Recovery Proxy (r:1 w:0)
	/// Proof: Recovery Proxy (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	fn as_recovered() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `281`
		//  Estimated: `3545`
		// Minimum execution time: 10_466_000 picoseconds.
		Weight::from_parts(10_670_000, 3545)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: Recovery Proxy (r:0 w:1)
	/// Proof: Recovery Proxy (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	fn set_recovered() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 10_297_000 picoseconds.
		Weight::from_parts(10_840_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Recovery Recoverable (r:1 w:1)
	/// Proof: Recovery Recoverable (max_values: None, max_size: Some(351), added: 2826, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn create_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `175`
		//  Estimated: `3816`
		// Minimum execution time: 28_510_000 picoseconds.
		Weight::from_parts(30_060_981, 3816)
			// Standard Error: 7_424
			.saturating_add(Weight::from_parts(56_594, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Recovery Recoverable (r:1 w:0)
	/// Proof: Recovery Recoverable (max_values: None, max_size: Some(351), added: 2826, mode: MaxEncodedLen)
	/// Storage: Recovery ActiveRecoveries (r:1 w:1)
	/// Proof: Recovery ActiveRecoveries (max_values: None, max_size: Some(389), added: 2864, mode: MaxEncodedLen)
	fn initiate_recovery() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `272`
		//  Estimated: `7670`
		// Minimum execution time: 31_478_000 picoseconds.
		Weight::from_parts(32_960_000, 7670)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Recovery Recoverable (r:1 w:0)
	/// Proof: Recovery Recoverable (max_values: None, max_size: Some(351), added: 2826, mode: MaxEncodedLen)
	/// Storage: Recovery ActiveRecoveries (r:1 w:1)
	/// Proof: Recovery ActiveRecoveries (max_values: None, max_size: Some(389), added: 2864, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn vouch_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `360 + n * (64 ±0)`
		//  Estimated: `7670`
		// Minimum execution time: 20_501_000 picoseconds.
		Weight::from_parts(21_889_392, 7670)
			// Standard Error: 6_732
			.saturating_add(Weight::from_parts(206_317, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Recovery Recoverable (r:1 w:0)
	/// Proof: Recovery Recoverable (max_values: None, max_size: Some(351), added: 2826, mode: MaxEncodedLen)
	/// Storage: Recovery ActiveRecoveries (r:1 w:0)
	/// Proof: Recovery ActiveRecoveries (max_values: None, max_size: Some(389), added: 2864, mode: MaxEncodedLen)
	/// Storage: Recovery Proxy (r:1 w:1)
	/// Proof: Recovery Proxy (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn claim_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `392 + n * (64 ±0)`
		//  Estimated: `11215`
		// Minimum execution time: 25_601_000 picoseconds.
		Weight::from_parts(27_004_521, 11215)
			// Standard Error: 7_284
			.saturating_add(Weight::from_parts(109_212, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Recovery ActiveRecoveries (r:1 w:1)
	/// Proof: Recovery ActiveRecoveries (max_values: None, max_size: Some(389), added: 2864, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn close_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `513 + n * (32 ±0)`
		//  Estimated: `7447`
		// Minimum execution time: 35_970_000 picoseconds.
		Weight::from_parts(37_632_353, 7447)
			// Standard Error: 8_484
			.saturating_add(Weight::from_parts(213_623, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: Recovery ActiveRecoveries (r:1 w:0)
	/// Proof: Recovery ActiveRecoveries (max_values: None, max_size: Some(389), added: 2864, mode: MaxEncodedLen)
	/// Storage: Recovery Recoverable (r:1 w:1)
	/// Proof: Recovery Recoverable (max_values: None, max_size: Some(351), added: 2826, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 9]`.
	fn remove_recovery(n: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `270 + n * (32 ±0)`
		//  Estimated: `7670`
		// Minimum execution time: 33_812_000 picoseconds.
		Weight::from_parts(35_604_811, 7670)
			// Standard Error: 10_784
			.saturating_add(Weight::from_parts(70_892, 0).saturating_mul(n.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: Recovery Proxy (r:1 w:1)
	/// Proof: Recovery Proxy (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	fn cancel_recovered() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `281`
		//  Estimated: `3545`
		// Minimum execution time: 12_545_000 picoseconds.
		Weight::from_parts(13_092_000, 3545)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
