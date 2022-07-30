use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Map, Value};

use crate::model::pagination::PageInfo;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct CosmosValidator {
  commission: Value,
  consensus_pubkey: Value,
  delegator_shares: String,
  description: Value,
  jailed: bool,
  min_self_delegation: String,
  operator_address: String,
  status: String,
  tokens: String,
  unbonding_height: String,
  unbonding_time: String,
}

impl CosmosValidator {
  pub fn new(value: Value) -> Self {
    serde_json::from_value::<Self>(value).unwrap()
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct PaginatedCosmosValidator {
  page_info: PageInfo,
  records: Vec<CosmosValidator>,
}

impl PaginatedCosmosValidator {
  pub fn new(value: Map<String, Value>, page: i64, count: i64) -> Self {
    let validators_opt = value.get("validators");
    if validators_opt.is_none() || validators_opt.unwrap().as_array().unwrap().len() == 0 {
      return Self {
        page_info: PageInfo::new(1, 0, 0, 0),
        records: Vec::new(),
      };
    }
    let validators = validators_opt.unwrap().as_array().unwrap();
    let mut records: Vec<CosmosValidator> = Vec::new();
    for validator in validators {
      records.push(serde_json::from_value::<CosmosValidator>(validator.clone()).unwrap());
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
