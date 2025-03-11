use alloy_primitives::B256;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use ssz_rs::prelude::*;
use tree_hash_derive::TreeHash;

#[derive(
    Debug, Eq, Hash, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize, Encode, Decode, TreeHash, SimpleSerialize,
)]
pub struct Checkpoint {
    pub epoch: u64,
    pub root: B256,
}
