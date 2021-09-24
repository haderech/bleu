use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct EthBlock {
    eth_blocks_id: i32,
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

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct EthTx {
    eth_txs_id: i32,
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