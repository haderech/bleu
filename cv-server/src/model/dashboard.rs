use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, Map, Value};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct Inflation {
  inflation: Option<String>,
}

impl Inflation {
  pub fn new(value: Map<String, Value>) -> Self {
    let inflation_opt = value.get("inflation");
    if inflation_opt.is_none() {
      return Self {
        inflation: None,
      };
    }
    let inflation = inflation_opt.unwrap();
    Self {
      inflation: Some(inflation.as_str().unwrap().to_string()),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct StakingPool {
  not_bonded_tokens: Option<String>,
  bonded_tokens: Option<String>,
}

impl StakingPool {
  pub fn new(value: Map<String, Value>) -> Self {
    let pool_opt = value.get("pool");
    if pool_opt.is_none() {
      return Self {
        not_bonded_tokens: None,
        bonded_tokens: None,
      };
    }
    let pool = pool_opt.unwrap();
    Self {
      not_bonded_tokens: Some(pool.as_object().unwrap().get("not_bonded_tokens").unwrap().as_str().unwrap().to_string()),
      bonded_tokens: Some(pool.as_object().unwrap().get("bonded_tokens").unwrap().as_str().unwrap().to_string()),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct CommunityPool {
  denom: Option<String>,
  amount: Option<String>,
}

impl CommunityPool {
  pub fn new(value: Map<String, Value>) -> Self {
    let pool_opt = value.get("pool");
    if pool_opt.is_none() {
      return Self {
        denom: None,
        amount: None,
      };
    }
    let pool = pool_opt.unwrap();
    let community_pool = pool.as_array().unwrap().get(0).unwrap().as_object().unwrap();
    Self {
      denom: Some(community_pool.get("denom").unwrap().as_str().unwrap().to_string()),
      amount: Some(community_pool.get("amount").unwrap().as_str().unwrap().to_string()),
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct NumOfVotingProposals {
  num_of_voting_proposals: i64,
}

impl NumOfVotingProposals {
  pub fn new(value: Map<String, Value>) -> Self {
    let pagination_opt = value.get("pagination");
    if pagination_opt.is_none() {
      return Self {
        num_of_voting_proposals: 0,
      };
    }
    let pagination = pagination_opt.unwrap();
    let total = pagination.as_object().unwrap().get("total").unwrap().as_str().unwrap().to_string();
    let total_count = from_str::<i64>(total.as_str()).unwrap();
    Self {
      num_of_voting_proposals: total_count,
    }
  }
}
