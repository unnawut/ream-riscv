use alloy_primitives::B256;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use ssz_rs::prelude::*;
use tree_hash_derive::TreeHash;

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Encode, Decode, TreeHash, SimpleSerialize)]
pub struct Eth1Data {
    pub deposit_root: B256,
    pub deposit_count: u64,
    pub block_hash: B256,
}
