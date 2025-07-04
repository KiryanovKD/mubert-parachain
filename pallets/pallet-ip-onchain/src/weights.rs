
//! Autogenerated weights for `pallet_ip_onchain`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 47.1.0
//! DATE: 2025-06-01, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! CPU: `13th Gen Intel(R) Core(TM) i7-13700HX`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: `1024`

// Executed Command:
// frame-omni-bencher
// v1
// benchmark
// pallet
// --runtime
// ./target/release/wbuild/mubert-runtime/mubert_runtime.wasm
// --pallet
// pallet_ip_onchain
// --extrinsic
// 
// --template
// ./pallets/benchmarking/frame-weight-template.hbs
// --output
// ./pallets/pallet-ip-onchain/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use polkadot_sdk::*;

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_ip_onchain`.
pub trait WeightInfo {
	fn create_author() -> Weight;
	fn edit_author() -> Weight;
	fn create_authority() -> Weight;
	fn edit_authority() -> Weight;
	fn create_entity() -> Weight;
	fn edit_entity() -> Weight;
	fn create_account_access() -> Weight;
	fn edit_account_access() -> Weight;
}

/// Weights for `pallet_ip_onchain` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Membership::Members` (r:1 w:0)
	/// Proof: `Membership::Members` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::NextAuthorId` (r:1 w:1)
	/// Proof: `IPOnchain::NextAuthorId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::Authors` (r:1 w:1)
	/// Proof: `IPOnchain::Authors` (`max_values`: None, `max_size`: Some(216), added: 2691, mode: `MaxEncodedLen`)
	fn create_author() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `207`
		//  Estimated: `4687`
		// Minimum execution time: 12_048_000 picoseconds.
		Weight::from_parts(12_927_000, 4687)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Membership::Members` (r:1 w:0)
	/// Proof: `Membership::Members` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::Authors` (r:1 w:1)
	/// Proof: `IPOnchain::Authors` (`max_values`: None, `max_size`: Some(216), added: 2691, mode: `MaxEncodedLen`)
	fn edit_author() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `291`
		//  Estimated: `4687`
		// Minimum execution time: 12_636_000 picoseconds.
		Weight::from_parts(13_183_000, 4687)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Membership::Members` (r:1 w:0)
	/// Proof: `Membership::Members` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::NextAuthorityId` (r:1 w:1)
	/// Proof: `IPOnchain::NextAuthorityId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::Authorities` (r:1 w:1)
	/// Proof: `IPOnchain::Authorities` (`max_values`: None, `max_size`: Some(54), added: 2529, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::AuthoritiesAccess` (r:1 w:1)
	/// Proof: `IPOnchain::AuthoritiesAccess` (`max_values`: None, `max_size`: Some(76), added: 2551, mode: `MaxEncodedLen`)
	fn create_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `207`
		//  Estimated: `4687`
		// Minimum execution time: 17_595_000 picoseconds.
		Weight::from_parts(18_522_000, 4687)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Membership::Members` (r:1 w:0)
	/// Proof: `Membership::Members` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::AuthoritiesAccess` (r:1 w:0)
	/// Proof: `IPOnchain::AuthoritiesAccess` (`max_values`: None, `max_size`: Some(76), added: 2551, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::Authorities` (r:1 w:1)
	/// Proof: `IPOnchain::Authorities` (`max_values`: None, `max_size`: Some(54), added: 2529, mode: `MaxEncodedLen`)
	fn edit_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `353`
		//  Estimated: `4687`
		// Minimum execution time: 16_589_000 picoseconds.
		Weight::from_parts(17_172_000, 4687)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Membership::Members` (r:1 w:0)
	/// Proof: `Membership::Members` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::AuthoritiesAccess` (r:1 w:0)
	/// Proof: `IPOnchain::AuthoritiesAccess` (`max_values`: None, `max_size`: Some(76), added: 2551, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::NextEntityId` (r:1 w:1)
	/// Proof: `IPOnchain::NextEntityId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::Entities` (r:1 w:1)
	/// Proof: `IPOnchain::Entities` (`max_values`: None, `max_size`: Some(610), added: 3085, mode: `MaxEncodedLen`)
	fn create_entity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `343`
		//  Estimated: `4687`
		// Minimum execution time: 17_402_000 picoseconds.
		Weight::from_parts(18_077_000, 4687)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Membership::Members` (r:1 w:0)
	/// Proof: `Membership::Members` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::Entities` (r:1 w:1)
	/// Proof: `IPOnchain::Entities` (`max_values`: None, `max_size`: Some(610), added: 3085, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::AuthoritiesAccess` (r:1 w:0)
	/// Proof: `IPOnchain::AuthoritiesAccess` (`max_values`: None, `max_size`: Some(76), added: 2551, mode: `MaxEncodedLen`)
	fn edit_entity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `405`
		//  Estimated: `4687`
		// Minimum execution time: 16_957_000 picoseconds.
		Weight::from_parts(17_252_000, 4687)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Membership::Members` (r:1 w:0)
	/// Proof: `Membership::Members` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::AuthoritiesAccess` (r:2 w:1)
	/// Proof: `IPOnchain::AuthoritiesAccess` (`max_values`: None, `max_size`: Some(76), added: 2551, mode: `MaxEncodedLen`)
	fn create_account_access() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `343`
		//  Estimated: `6092`
		// Minimum execution time: 16_722_000 picoseconds.
		Weight::from_parts(17_806_000, 6092)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Membership::Members` (r:1 w:0)
	/// Proof: `Membership::Members` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::AuthoritiesAccess` (r:1 w:1)
	/// Proof: `IPOnchain::AuthoritiesAccess` (`max_values`: None, `max_size`: Some(76), added: 2551, mode: `MaxEncodedLen`)
	fn edit_account_access() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `343`
		//  Estimated: `4687`
		// Minimum execution time: 16_385_000 picoseconds.
		Weight::from_parts(16_983_000, 4687)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Membership::Members` (r:1 w:0)
	/// Proof: `Membership::Members` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::NextAuthorId` (r:1 w:1)
	/// Proof: `IPOnchain::NextAuthorId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::Authors` (r:1 w:1)
	/// Proof: `IPOnchain::Authors` (`max_values`: None, `max_size`: Some(216), added: 2691, mode: `MaxEncodedLen`)
	fn create_author() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `207`
		//  Estimated: `4687`
		// Minimum execution time: 12_048_000 picoseconds.
		Weight::from_parts(12_927_000, 4687)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Membership::Members` (r:1 w:0)
	/// Proof: `Membership::Members` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::Authors` (r:1 w:1)
	/// Proof: `IPOnchain::Authors` (`max_values`: None, `max_size`: Some(216), added: 2691, mode: `MaxEncodedLen`)
	fn edit_author() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `291`
		//  Estimated: `4687`
		// Minimum execution time: 12_636_000 picoseconds.
		Weight::from_parts(13_183_000, 4687)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Membership::Members` (r:1 w:0)
	/// Proof: `Membership::Members` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::NextAuthorityId` (r:1 w:1)
	/// Proof: `IPOnchain::NextAuthorityId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::Authorities` (r:1 w:1)
	/// Proof: `IPOnchain::Authorities` (`max_values`: None, `max_size`: Some(54), added: 2529, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::AuthoritiesAccess` (r:1 w:1)
	/// Proof: `IPOnchain::AuthoritiesAccess` (`max_values`: None, `max_size`: Some(76), added: 2551, mode: `MaxEncodedLen`)
	fn create_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `207`
		//  Estimated: `4687`
		// Minimum execution time: 17_595_000 picoseconds.
		Weight::from_parts(18_522_000, 4687)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Membership::Members` (r:1 w:0)
	/// Proof: `Membership::Members` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::AuthoritiesAccess` (r:1 w:0)
	/// Proof: `IPOnchain::AuthoritiesAccess` (`max_values`: None, `max_size`: Some(76), added: 2551, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::Authorities` (r:1 w:1)
	/// Proof: `IPOnchain::Authorities` (`max_values`: None, `max_size`: Some(54), added: 2529, mode: `MaxEncodedLen`)
	fn edit_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `353`
		//  Estimated: `4687`
		// Minimum execution time: 16_589_000 picoseconds.
		Weight::from_parts(17_172_000, 4687)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Membership::Members` (r:1 w:0)
	/// Proof: `Membership::Members` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::AuthoritiesAccess` (r:1 w:0)
	/// Proof: `IPOnchain::AuthoritiesAccess` (`max_values`: None, `max_size`: Some(76), added: 2551, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::NextEntityId` (r:1 w:1)
	/// Proof: `IPOnchain::NextEntityId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::Entities` (r:1 w:1)
	/// Proof: `IPOnchain::Entities` (`max_values`: None, `max_size`: Some(610), added: 3085, mode: `MaxEncodedLen`)
	fn create_entity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `343`
		//  Estimated: `4687`
		// Minimum execution time: 17_402_000 picoseconds.
		Weight::from_parts(18_077_000, 4687)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `Membership::Members` (r:1 w:0)
	/// Proof: `Membership::Members` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::Entities` (r:1 w:1)
	/// Proof: `IPOnchain::Entities` (`max_values`: None, `max_size`: Some(610), added: 3085, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::AuthoritiesAccess` (r:1 w:0)
	/// Proof: `IPOnchain::AuthoritiesAccess` (`max_values`: None, `max_size`: Some(76), added: 2551, mode: `MaxEncodedLen`)
	fn edit_entity() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `405`
		//  Estimated: `4687`
		// Minimum execution time: 16_957_000 picoseconds.
		Weight::from_parts(17_252_000, 4687)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Membership::Members` (r:1 w:0)
	/// Proof: `Membership::Members` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::AuthoritiesAccess` (r:2 w:1)
	/// Proof: `IPOnchain::AuthoritiesAccess` (`max_values`: None, `max_size`: Some(76), added: 2551, mode: `MaxEncodedLen`)
	fn create_account_access() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `343`
		//  Estimated: `6092`
		// Minimum execution time: 16_722_000 picoseconds.
		Weight::from_parts(17_806_000, 6092)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Membership::Members` (r:1 w:0)
	/// Proof: `Membership::Members` (`max_values`: Some(1), `max_size`: Some(3202), added: 3697, mode: `MaxEncodedLen`)
	/// Storage: `IPOnchain::AuthoritiesAccess` (r:1 w:1)
	/// Proof: `IPOnchain::AuthoritiesAccess` (`max_values`: None, `max_size`: Some(76), added: 2551, mode: `MaxEncodedLen`)
	fn edit_account_access() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `343`
		//  Estimated: `4687`
		// Minimum execution time: 16_385_000 picoseconds.
		Weight::from_parts(16_983_000, 4687)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
