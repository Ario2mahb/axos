//! Test utilities for the primitives crate.
//!
//! This module is only available when the `test-utils` feature is enabled.
use crate::blocks::BlockWithTransactions;

/// Returns a valid example block with transactions.
pub fn valid_block() -> Option<BlockWithTransactions> {
    // let raw_block = r#"{
    //     "hash": "0x2e4f4aff36bb7951be9742ad349fb1db84643c6bbac5014f3d196fd88fe333eb",
    //     "parentHash": "0xeccf4c06ad0d27be1cadee5720a509d31a9de0462b52f2cf6045d9a73c9aa504",
    //     "sha3Uncles": "0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347",
    //     "miner": "0x4200000000000000000000000000000000000011",
    //     "stateRoot": "0x5905b2423f299a29db41e377d7ceadf4baa49eed04e1b72957e8c0985e04e730",
    //     "transactionsRoot": "0x030e481411042a769edde83d790d583ed69f9d3098d4a78d00e008f749fcfd97",
    //     "receiptsRoot": "0x29079b696c12a19999f3bb303fddb6fc12fb701f427678cca24954b91080ada3",
    //     "number": "0x7fe52f",
    //     "gasUsed": "0xb711",
    //     "gasLimit": "0x17d7840",
    //     "extraData": "0x",
    //     "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000",
    //     "timestamp": "0x644434c2",
    //     "difficulty": "0x0",
    //     "totalDifficulty": "0x0",
    //     "sealFields": [],
    //     "uncles": [],
    //     "transactions": [],
    //     "size": "0x365",
    //     "mixHash": "0x7aeec5550a9b0616701e49ab835af5f10eadba2a0582016f0e256c9cace0c046",
    //     "nonce": "0x0000000000000000",
    //     "baseFeePerGas": "0x32"
    // }
    // "#;
    // serde_json::from_str(raw_block).ok()
    None
}
