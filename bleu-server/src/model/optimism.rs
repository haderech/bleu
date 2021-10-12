use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct OptimismBatchSummary {
    batch_index: Option<String>,
    l1_tx_hash: Option<String>,
    tx_size: Option<i32>,
    timestamp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct OptimismBatch {
    optimism_batches_id: i32,
    batch_index: Option<String>,
    timestamp: Option<String>,
    batch_size: Option<String>,
    l1_tx_hash: Option<String>,
    l1_block_number: Option<String>,
    batch_root: Option<String>,
    previous_total_elements: Option<String>,
    extra_data: Option<String>,
    tx_size: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct OptimismTxSummary {
    tx_hash: Option<String>,
    from_address: Option<String>,
    to_address: Option<String>,
    value: Option<String>,
    timestamp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct OptimismTx {
    optimism_txs_id: i32,
    tx_hash: Option<String>,
    status: Option<String>,
    tx_index: Option<String>,
    timestamp: Option<String>,
    from_address: Option<String>,
    to_address: Option<String>,
    token_transferred: Option<String>,
    value: Option<String>,
    tx_fee: Option<String>,
    ether_price: Option<String>,
    gas_used_by_tx: Option<String>,
    nonce: Option<String>,
    input_data: Option<String>,
    l1_txn_batch_index: Option<String>,
    l1_submission_tx_hash: Option<String>,
    l1_state_batch_index: Option<String>,
    l1_state_root_submission_tx_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct OptimismL1ToL2Tx {
    optimism_l1_to_l2_txs_id: i32,
    l1_block_number: Option<String>,
    l1_tx_hash: Option<String>,
    l2_tx_hash: Option<String>,
}