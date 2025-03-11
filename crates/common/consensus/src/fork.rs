use alloy_primitives::aliases::B32;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use ssz_rs::prelude::*;
use tree_hash_derive::TreeHash;

#[derive(Debug, PartialEq, Clone, Copy, serde::Serialize, serde::Deserialize, Encode, Decode, TreeHash, SimpleSerialize)]
pub struct Fork {
    pub previous_version: B32,
    pub current_version: B32,
    pub epoch: u64,
}
