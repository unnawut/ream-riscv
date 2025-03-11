use alloy_primitives::B256;
use ream_bls::BLSSignature;
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use ssz_rs::prelude::*;
use tree_hash_derive::TreeHash;

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Encode, Decode, TreeHash, SimpleSerialize)]
pub struct SignedBeaconBlockHeader {
    pub message: BeaconBlockHeader,
    pub signature: BLSSignature,
}

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Encode, Decode, TreeHash, SimpleSerialize)]
pub struct BeaconBlockHeader {
    pub slot: u64,
    pub proposer_index: u64,
    pub parent_root: B256,
    pub state_root: B256,
    pub body_root: B256,
}
