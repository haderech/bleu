use diesel::Insertable;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::libs::serde::find_value_string;

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "eth_blocks"]
pub struct NewEthBlock {
    base_fee_per_gas: String,
    difficulty: String,
    extra_data: String,
    gas_limit: String,
    gas_used: String,
    hash: String,
    logs_bloom: String,
    miner: String,
    mix_hash: String,
    nonce: String,
    block_number: String,
    parent_hash: String,
    receipts_root: String,
    sha3_uncles: String,
    block_size: String,
    state_root: String,
    block_timestamp: String,
    total_difficulty: String,
    is_forked: bool,
}

impl NewEthBlock {
    pub fn new(raw_eth_block: &Map<String, Value>) -> NewEthBlock {
        NewEthBlock {
            base_fee_per_gas: find_value_string(raw_eth_block, "baseFeePerGas"),
            difficulty: find_value_string(raw_eth_block, "difficulty"),
            extra_data: find_value_string(raw_eth_block, "extraData"),
            gas_limit: find_value_string(raw_eth_block, "gasLimit"),
            gas_used: find_value_string(raw_eth_block, "gasUsed"),
            hash: find_value_string(raw_eth_block, "hash"),
            logs_bloom: find_value_string(raw_eth_block, "logsBloom"),
            miner: find_value_string(raw_eth_block, "miner"),
            mix_hash: find_value_string(raw_eth_block, "mixHash"),
            nonce: find_value_string(raw_eth_block, "nonce"),
            block_number: find_value_string(raw_eth_block, "number"),
            parent_hash: find_value_string(raw_eth_block, "parentHash"),
            receipts_root: find_value_string(raw_eth_block, "receiptsRoot"),
            sha3_uncles: find_value_string(raw_eth_block, "sha3Uncles"),
            block_size: find_value_string(raw_eth_block, "size"),
            state_root: find_value_string(raw_eth_block, "stateRoot"),
            block_timestamp: find_value_string(raw_eth_block, "timestamp"),
            total_difficulty: find_value_string(raw_eth_block, "totalDifficulty"),
            is_forked: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable)]
#[table_name = "eth_txs"]
pub struct NewEthTx {
    block_hash: String,
    block_number: String,
    chain_id: String,
    from_address: String,
    to_address: String,
    gas: String,
    gas_price: String,
    hash: String,
    tx_input: String,
    max_fee_per_gas: String,
    max_priority_fee_per_gas: String,
    nonce: String,
    tx_index: String,
    value: String,
    is_forked: bool,
}

impl NewEthTx {
    pub fn new(raw_eth_tx: &Map<String, Value>) -> NewEthTx {
        NewEthTx {
            block_hash: find_value_string(raw_eth_tx, "blockHash"),
            block_number: find_value_string(raw_eth_tx, "blockNumber"),
            chain_id: find_value_string(raw_eth_tx, "chainId"),
            from_address: find_value_string(raw_eth_tx, "from"),
            to_address: find_value_string(raw_eth_tx, "to"),
            gas: find_value_string(raw_eth_tx, "gas"),
            gas_price: find_value_string(raw_eth_tx, "gasPrice"),
            hash: find_value_string(raw_eth_tx, "hash"),
            tx_input: find_value_string(raw_eth_tx, "input"),
            max_fee_per_gas: find_value_string(raw_eth_tx, "maxFeePerGas"),
            max_priority_fee_per_gas: find_value_string(raw_eth_tx, "maxPriorityFeePerGas"),
            nonce: find_value_string(raw_eth_tx, "nonce"),
            tx_index: find_value_string(raw_eth_tx, "transactionIndex"),
            value: find_value_string(raw_eth_tx, "value"),
            is_forked: false,
        }
    }
}

table! {
    eth_blocks (eth_blocks_id) {
        eth_blocks_id -> Integer,
        base_fee_per_gas -> Text,
        difficulty -> Text,
        extra_data -> Text,
        gas_limit -> Text,
        gas_used -> Text,
        hash -> Text,
        logs_bloom -> Text,
        miner -> Text,
        mix_hash -> Text,
        nonce -> Text,
        block_number -> Text,
        parent_hash -> Text,
        receipts_root -> Text,
        sha3_uncles -> Text,
        block_size -> Text,
        state_root -> Text,
        block_timestamp -> Text,
        total_difficulty -> Text,
        is_forked -> Bool,
    }
}

table! {
    eth_txs (eth_txs_id) {
        eth_txs_id -> Integer,
        block_hash -> Text,
        block_number -> Text,
        chain_id -> Text,
        from_address -> Text,
        to_address -> Text,
        gas -> Text,
        gas_price -> Text,
        hash -> Text,
        tx_input -> Text,
        max_fee_per_gas -> Text,
        max_priority_fee_per_gas -> Text,
        nonce -> Text,
        tx_index -> Text,
        value -> Text,
        is_forked -> Bool,
    }
}