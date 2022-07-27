use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

use crate::model::pagination::PageInfo;
use crate::repository::pagination::PaginatedRecord;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct CosmosBlock {
  cosmos_block_id: i64,
  app_hash: Option<String>,
  chain_id: Option<String>,
  consensus_hash: Option<String>,
  data_hash: Option<String>,
  evidence_hash: Option<String>,
  hash: Option<String>,
  height: Option<String>,
  last_block_id: Option<String>,
  last_commit_hash: Option<String>,
  last_results_hash: Option<String>,
  next_validators_hash: Option<String>,
  num_txs: i64,
  proposer_address: Option<String>,
  time: Option<String>,
  validators_hash: Option<String>,
  version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct PaginatedCosmosBlock {
  page_info: PageInfo,
  records: Vec<CosmosBlock>,
}

impl PaginatedCosmosBlock {
  pub fn new(paginated: PaginatedRecord<CosmosBlock>) -> Self {
    Self {
      page_info: PageInfo::new(paginated.page, paginated.count, paginated.total_page, paginated.total_count),
      records: paginated.records,
    }
  }
}
