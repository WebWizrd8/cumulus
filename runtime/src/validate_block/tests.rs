// Copyright 2019 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

use crate::{ParachainBlockData, WitnessData};

use rio::TestExternalities;
use keyring::AccountKeyring;
use primitives::{storage::well_known_keys};
use runtime_primitives::traits::{Block as BlockT, Header as HeaderT};
use executor::{WasmExecutor, error::Result, wasmi::RuntimeValue::{I64, I32}};
use test_client::{
	TestClientBuilder, TestClient,
	runtime::{Block, Transfer}, TestClientBuilderExt,
};

use std::collections::HashMap;

use codec::Encode;

const WASM_CODE: &[u8] =
	include_bytes!("../../../test/runtime/wasm/target/wasm32-unknown-unknown/release/cumulus_test_runtime.compact.wasm");

fn call_validate_block(block_data: ParachainBlockData<Block>) -> Result<()> {
	let mut ext = TestExternalities::default();
	WasmExecutor::new().call_with_custom_signature(
		&mut ext,
		1024,
		&WASM_CODE,
		"validate_block",
		|alloc| {
			let block_data = block_data.encode();
			println!("ALLOC: {}", block_data.len());
			let block_data_offset = alloc(&block_data)?;
			println!("ALLOC");

			Ok(
				vec![
					I32(block_data_offset as i32),
					I64(block_data.len() as i64),
				]
			)
		},
		|res, _| {
			if res.is_none() {
				Ok(Some(()))
			} else {
				Ok(None)
			}
		}
	)
}

fn create_extrinsics() -> Vec<<Block as BlockT>::Extrinsic> {
	vec![
		Transfer {
			from: AccountKeyring::Alice.into(),
			to: AccountKeyring::Bob.into(),
			amount: 69,
			nonce: 0,
		}.into_signed_tx()
	]
}

fn create_test_client() -> TestClient {
	let mut genesis_extension = HashMap::new();
	genesis_extension.insert(well_known_keys::CODE.to_vec(), WASM_CODE.to_vec());

	TestClientBuilder::new()
		.set_genesis_extension(genesis_extension)
		.build_cumulus()
}

fn build_block_with_proof(
	client: &TestClient,
	extrinsics: Vec<<Block as BlockT>::Extrinsic>,
) -> (Block, WitnessData) {
	let mut builder = client.new_block().expect("Initializes new block");
	builder.record_proof();

	extrinsics.into_iter().for_each(|e| builder.push(e).expect("Pushes an extrinsic"));

	let (block, proof) = builder
		.bake_and_extract_proof()
		.expect("Finalizes block");

	(block, proof.expect("We enabled proof recording before."))
}

#[test]
fn validate_block_with_empty_block() {
	let client = create_test_client();
	let witness_data_storage_root = *client
		.best_block_header()
		.expect("Best block exists")
		.state_root();
	let (block, witness_data) = build_block_with_proof(&client, Vec::new());
	let (header, extrinsics) = block.deconstruct();

	let block_data = ParachainBlockData::new(
		header,
		extrinsics,
		witness_data,
		witness_data_storage_root
	);
	call_validate_block(block_data).expect("Calls `validate_block`");
}

// #[test]
// fn validate_block_with_empty_witness_data() {
// 	let prev_header = create_header();

// 	let block = ParachainBlock::new(create_extrinsics(), Default::default());
// 	assert!(call_validate_block(block).is_err());
// }

// #[test]
// fn validate_block_with_witness_data() {
// 	let prev_header = create_header();

// 	let block = ParachainBlock::new(create_extrinsics(), create_witness_data());
// 	call_validate_block(block).expect("`validate_block` succeeds");
// }