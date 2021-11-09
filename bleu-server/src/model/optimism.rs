use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

use crate::libs;
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

impl OptimismTxBatch {
    pub fn get_batch_index(&self) -> String {
        if self.batch_index.is_none() {
            String::from("0")
        } else {
            self.batch_index.clone().unwrap()
        }
    }
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
pub struct OptimismBlockTxExtended {
    optimism_block_tx: OptimismBlockTx,
    l1_tx_batch_index: Option<String>,
    l1_submission_tx_hash: Option<String>,
    l1_state_batch_index: Option<String>,
    l1_state_root_submission_tx_hash: Option<String>,
    state: Option<String>,
}

impl OptimismBlockTxExtended {
    pub fn is_l1_origin(&self) -> bool {
        self.optimism_block_tx.queue_origin.is_some() && self.optimism_block_tx.queue_origin.clone().unwrap() == "l1".to_string()
    }

    pub fn get_queue_index(&self) -> Option<String> {
        self.optimism_block_tx.queue_index.clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct OptimismBlockTxDetail {
    optimism_block_tx_ext: OptimismBlockTxExtended,
    l1_origin_tx_hash: Option<String>,
}

impl OptimismBlockTxDetail {
    pub fn from(optimism_block_tx_ext: OptimismBlockTxExtended, l1_origin_tx_hash: Option<String>) -> Self {
        Self {
            optimism_block_tx_ext,
            l1_origin_tx_hash,
        }
    }
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

impl OptimismStateBatch {
    pub fn get_batch_index(&self) -> String {
        if self.batch_index.is_none() {
            String::from("0")
        } else {
            self.batch_index.clone().unwrap()
        }
    }
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
pub struct OptimismL1ToL2TxSummary {
    l1_block_number: Option<String>,
    l1_tx_hash: Option<String>,
    l2_tx_hash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct OptimismL1ToL2Tx {
    l1_block_number: Option<String>,
    queue_index: Option<String>,
    l2_tx_hash: Option<String>,
    timestamp: Option<String>,
    l1_tx_hash: Option<String>,
    l1_tx_origin: Option<String>,
    gas_limit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct PaginatedOptimismL1ToL2Tx {
    page_info: PageInfo,
    records: Vec<OptimismL1ToL2Tx>,
}

impl PaginatedOptimismL1ToL2Tx {
    pub fn new(paginated: PaginatedRecord<OptimismL1ToL2Tx>) -> Self {
        Self {
            page_info: PageInfo::new(paginated.page, paginated.count, paginated.total_page, paginated.total_count),
            records: paginated.records,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct OptimismTxReceiptLog {
    optimism_tx_receipt_logs_id: i64,
    address: Option<String>,
    topics: Option<String>,
    data: Option<String>,
    block_number: Option<String>,
    tx_hash: Option<String>,
    tx_index: Option<String>,
    block_hash: Option<String>,
    log_index: Option<String>,
    removed: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Apiv2Schema)]
pub struct OptimismTxLog {
    address: Option<String>,
    topics: Option<Vec<String>>,
    data: Option<String>,
    block_number: Option<String>,
    tx_hash: Option<String>,
    tx_index: Option<String>,
    block_hash: Option<String>,
    log_index: Option<String>,
    removed: Option<bool>,
}

impl OptimismTxLog {
    pub fn from(e: OptimismTxReceiptLog) -> Self {
        let topic_vec = match e.topics {
            None => None,
            Some(topics) => Some(libs::convert::convert_str_to_vec(topics))
        };
        Self {
            address: e.address,
            topics: topic_vec,
            data: e.data,
            block_number: e.block_number,
            tx_hash: e.tx_hash,
            tx_index: e.tx_index,
            block_hash: e.block_hash,
            log_index: e.log_index,
            removed: e.removed,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct BoardSummary {
    latest_tx_batch_index: String,
    latest_state_batch_index: String,
    tx_count: i64,
}

impl BoardSummary {
    pub fn new(
        latest_tx_batch_index: String,
        latest_state_batch_index: String,
        tx_count: i64,
    ) -> Self {
        Self {
            latest_tx_batch_index,
            latest_state_batch_index,
            tx_count,
        }
    }
}