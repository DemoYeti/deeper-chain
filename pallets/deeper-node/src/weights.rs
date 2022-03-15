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

//! Autogenerated weights for pallet_deeper_node
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-03-15, STEPS: 50, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/deeper-chain
// benchmark
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_deeper_node
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./pallets/deeper-node/src/weights.rs
// --template=./.maintain/frame-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_deeper_node.
pub trait WeightInfo {	fn register_device() -> Weight;	fn unregister_device() -> Weight;	fn register_server() -> Weight;	fn update_server() -> Weight;	fn unregister_server() -> Weight;	fn im_online() -> Weight;}

/// Weights for pallet_deeper_node using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {	fn register_device() -> Weight {
		(57_696_000 as Weight)			.saturating_add(T::DbWeight::get().reads(4 as Weight))			.saturating_add(T::DbWeight::get().writes(2 as Weight))	}	fn unregister_device() -> Weight {
		(74_621_000 as Weight)			.saturating_add(T::DbWeight::get().reads(7 as Weight))			.saturating_add(T::DbWeight::get().writes(2 as Weight))	}	fn register_server() -> Weight {
		(72_870_000 as Weight)			.saturating_add(T::DbWeight::get().reads(6 as Weight))			.saturating_add(T::DbWeight::get().writes(4 as Weight))	}	fn update_server() -> Weight {
		(22_641_000 as Weight)			.saturating_add(T::DbWeight::get().reads(1 as Weight))			.saturating_add(T::DbWeight::get().writes(1 as Weight))	}	fn unregister_server() -> Weight {
		(78_785_000 as Weight)			.saturating_add(T::DbWeight::get().reads(6 as Weight))			.saturating_add(T::DbWeight::get().writes(4 as Weight))	}	fn im_online() -> Weight {
		(24_975_000 as Weight)			.saturating_add(T::DbWeight::get().reads(2 as Weight))			.saturating_add(T::DbWeight::get().writes(3 as Weight))	}}

// For backwards compatibility and tests
impl WeightInfo for () {	fn register_device() -> Weight {
		(57_696_000 as Weight)			.saturating_add(RocksDbWeight::get().reads(4 as Weight))			.saturating_add(RocksDbWeight::get().writes(2 as Weight))	}	fn unregister_device() -> Weight {
		(74_621_000 as Weight)			.saturating_add(RocksDbWeight::get().reads(7 as Weight))			.saturating_add(RocksDbWeight::get().writes(2 as Weight))	}	fn register_server() -> Weight {
		(72_870_000 as Weight)			.saturating_add(RocksDbWeight::get().reads(6 as Weight))			.saturating_add(RocksDbWeight::get().writes(4 as Weight))	}	fn update_server() -> Weight {
		(22_641_000 as Weight)			.saturating_add(RocksDbWeight::get().reads(1 as Weight))			.saturating_add(RocksDbWeight::get().writes(1 as Weight))	}	fn unregister_server() -> Weight {
		(78_785_000 as Weight)			.saturating_add(RocksDbWeight::get().reads(6 as Weight))			.saturating_add(RocksDbWeight::get().writes(4 as Weight))	}	fn im_online() -> Weight {
		(24_975_000 as Weight)			.saturating_add(RocksDbWeight::get().reads(2 as Weight))			.saturating_add(RocksDbWeight::get().writes(3 as Weight))	}}
