use serde::{Deserialize, Serialize};

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
}
