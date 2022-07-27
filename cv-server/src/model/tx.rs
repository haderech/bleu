use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

use crate::model::pagination::PageInfo;
use crate::repository::pagination::PaginatedRecord;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct CosmosTx {
  cosmos_tx_id: i64,
  code: i64,
  data: Option<String>,
  fee: Option<serde_json::Value>,
  gas_used: Option<String>,
  gas_wanted: Option<String>,
  height: Option<String>,
  memo: Option<String>,
  messages: Option<serde_json::Value>,
  raw_log: Option<String>,
  timestamp: Option<String>,
  txhash: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct PaginatedCosmosTx {
  page_info: PageInfo,
  records: Vec<CosmosTx>,
}

impl PaginatedCosmosTx {
  pub fn new(paginated: PaginatedRecord<CosmosTx>) -> Self {
    Self {
      page_info: PageInfo::new(paginated.page, paginated.count, paginated.total_page, paginated.total_count),
      records: paginated.records,
    }
  }
}
