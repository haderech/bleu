use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Map, Value};

use crate::model::pagination::PageInfo;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct CosmosProposal {
  content: Value,
  deposit_end_time: String,
  final_tally_result: Value,
  proposal_id: String,
  status: String,
  submit_time: String,
  total_deposit: Value,
  voting_end_time: String,
  voting_start_time: String,
}

impl CosmosProposal {
  pub fn new(value: Value) -> Self {
    serde_json::from_value::<Self>(value).unwrap()
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct PaginatedCosmosProposal {
  page_info: PageInfo,
  records: Vec<CosmosProposal>,
}

impl PaginatedCosmosProposal {
  pub fn new(value: Map<String, Value>, page: i64, count: i64) -> Self {
    let proposals_opt = value.get("proposals");
    if proposals_opt.is_none() || proposals_opt.unwrap().as_array().unwrap().len() == 0 {
      return Self {
        page_info: PageInfo::new(1, 0, 0, 0),
        records: Vec::new(),
      };
    }
    let proposals = proposals_opt.unwrap().as_array().unwrap();
    let mut records: Vec<CosmosProposal> = Vec::new();
    for proposal in proposals {
      records.push(serde_json::from_value::<CosmosProposal>(proposal.clone()).unwrap());
    }

    let pagination = value.get("pagination").unwrap().as_object().unwrap();
    let total_str = pagination.get("total").unwrap().as_str().unwrap();
    let total = from_str::<i64>(total_str).unwrap();

    let total_page = if total % count == 0 {
      total / count
    } else {
      (total / count) + 1
    };
    Self {
      page_info: PageInfo::new(page, count, total_page, total),
      records,
    }
  }
}
