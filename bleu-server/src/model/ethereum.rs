use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct EthereumTxLog {
    ethereum_tx_log_id: i64,
    address: Option<String>,
    topics: Option<String>,
    data: Option<String>,
    block_number: Option<String>,
    tx_hash: Option<String>,
    tx_index: Option<String>,
    block_hash: Option<String>,
    log_index: Option<String>,
    removed: Option<bool>,
    queue_index: Option<String>,
}

impl EthereumTxLog {
    pub fn get_tx_hash(&self) -> Option<String> {
        self.tx_hash.clone()
    }
}