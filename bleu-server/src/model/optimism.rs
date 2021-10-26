use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

use crate::model::pagination::PageInfo;
use crate::repository::pagination::PaginatedRecord;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct OptimismTxBatchSummary {
    batch_index: Option<String>,
    l1_tx_hash: Option<String>,
    batch_size: Option<String>,
    batch_timestamp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct OptimismTxBatch {
    optimism_tx_batches_id: i64,
    batch_index: Option<String>,
    batch_timestamp: Option<String>,
    batch_size: Option<String>,
    l1_tx_hash: Option<String>,
    l1_block_number: Option<String>,
    batch_root: Option<String>,
    previous_total_elements: Option<String>,
    extra_data: Option<String>,
    submitter: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct PaginatedOptimismTxBatch {
    page_info: PageInfo,
    records: Vec<OptimismTxBatch>,
}

impl PaginatedOptimismTxBatch {
    pub fn new(paginated: PaginatedRecord<OptimismTxBatch>) -> Self {
        Self {
            page_info: PageInfo::new(paginated.page, paginated.count, paginated.total_page, paginated.total_count),
            records: paginated.records,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct OptimismTxSummary {
    tx_hash: Option<String>,
    from_address: Option<String>,
    to_address: Option<String>,
    value: Option<String>,
    tx_timestamp: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct OptimismTx {
    optimism_txs_id: i64,
    index: Option<String>,
    batch_index: Option<String>,
    block_number: Option<String>,
    tx_timestamp: Option<String>,
    gas_limit: Option<String>,
    target: Option<String>,
    origin: Option<String>,
    data: Option<String>,
    queue_origin: Option<String>,
    value: Option<String>,
    queue_index: Option<String>,
    decoded: Option<String>,
    confirmed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct OptimismBlockTx {
    optimism_block_txs_id: i64,
    block_hash: Option<String>,
    block_number: Option<String>,
    from_address: Option<String>,
    gas: Option<String>,
    gas_price: Option<String>,
    hash: Option<String>,
    index: Option<String>,
    tx_input: Option<String>,
    l1_block_number: Option<String>,
    l1_timestamp: Option<String>,
    l1_tx_origin: Option<String>,
    nonce: Option<String>,
    queue_index: Option<String>,
    queue_origin: Option<String>,
    raw_tx: Option<String>,
    to_address: Option<String>,
    tx_index: Option<String>,
    tx_type: Option<String>,
    value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct PaginatedOptimismBlockTx {
    page_info: PageInfo,
    records: Vec<OptimismBlockTx>,
}

impl PaginatedOptimismBlockTx {
    pub fn new(paginated: PaginatedRecord<OptimismBlockTx>) -> Self {
        Self {
            page_info: PageInfo::new(paginated.page, paginated.count, paginated.total_page, paginated.total_count),
            records: paginated.records,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct OptimismStateBatch {
    optimism_state_batches_id: i64,
    batch_index: Option<String>,
    batch_timestamp: Option<String>,
    batch_size: Option<String>,
    l1_tx_hash: Option<String>,
    l1_block_number: Option<String>,
    batch_root: Option<String>,
    previous_total_elements: Option<String>,
    extra_data: Option<String>,
    submitter: Option<String>,
}


#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct PaginatedOptimismStateBatch {
    page_info: PageInfo,
    records: Vec<OptimismStateBatch>,
}

impl PaginatedOptimismStateBatch {
    pub fn new(paginated: PaginatedRecord<OptimismStateBatch>) -> Self {
        Self {
            page_info: PageInfo::new(paginated.page, paginated.count, paginated.total_page, paginated.total_count),
            records: paginated.records,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct OptimismStateRoot {
    optimism_state_roots_id: i64,
    index: Option<String>,
    batch_index: Option<String>,
    value: Option<String>,
    confirmed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct OptimismL1ToL2Tx {
    l1_block_number: Option<String>,
    l1_tx_hash: Option<String>,
    l2_tx_hash: Option<String>,
}