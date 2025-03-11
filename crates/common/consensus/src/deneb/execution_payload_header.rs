use alloy_primitives::{Address, B256, U256};
use serde::{Deserialize, Serialize};
use ssz_derive::{Decode, Encode};
use ssz_rs::prelude::*;
use ssz_types::{
    serde_utils::{hex_fixed_vec, hex_var_list},
    typenum, FixedVector, VariableList,
};
use tree_hash_derive::TreeHash;

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize, Encode, Decode, TreeHash, SimpleSerialize)]
pub struct ExecutionPayloadHeader {
    // Execution block header fields
    pub parent_hash: B256,
    pub fee_recipient: Address,
    pub state_root: B256,
    pub receipts_root: B256,
    #[serde(with = "hex_fixed_vec")]
    pub logs_bloom: FixedVector<u8, typenum::U256>,
    pub prev_randao: B256,
    pub block_number: u64,
    pub gas_limit: u64,
    pub gas_used: u64,
    pub timestamp: u64,
    #[serde(with = "hex_var_list")]
    pub extra_data: VariableList<u8, typenum::U32>,
    #[serde(with = "serde_utils::quoted_u256")]
    pub base_fee_per_gas: U256,

    // Extra payload fields
    pub block_hash: B256,
    pub transactions_root: B256,
    pub withdrawals_root: B256,
    pub blob_gas_used: u64,
    pub excess_blob_gas: u64,
}
