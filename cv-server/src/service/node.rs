use std::env;

use cached::proc_macro::cached;
use serde_json::{Map, Value};

use crate::error::error::ExpectedError;
use crate::model::dashboard::{CommunityPool, Inflation, NumOfVotingProposals, StakingPool};
use crate::model::proposal::{CosmosProposal, PaginatedCosmosProposal};
use crate::model::validator::{CosmosValidator, PaginatedCosmosValidator};

#[cached(time = 60, key = "bool", convert = r#"{ true }"#, result = true)]
pub fn get_inflation() -> Result<Inflation, ExpectedError> {
  Ok(Inflation::new(request_to_node("/cosmos/mint/v1beta1/inflation")?))
}

#[cached(time = 60, key = "bool", convert = r#"{ true }"#, result = true)]
pub fn get_staking_pool() -> Result<StakingPool, ExpectedError> {
  Ok(StakingPool::new(request_to_node("/cosmos/staking/v1beta1/pool")?))
}

#[cached(time = 60, key = "bool", convert = r#"{ true }"#, result = true)]
pub fn get_community_pool() -> Result<CommunityPool, ExpectedError> {
  Ok(CommunityPool::new(request_to_node("/cosmos/distribution/v1beta1/community_pool")?))
}

#[cached(time = 60, key = "bool", convert = r#"{ true }"#, result = true)]
pub fn get_num_of_voting_proposals() -> Result<NumOfVotingProposals, ExpectedError> {
  Ok(NumOfVotingProposals::new(request_to_node("/cosmos/gov/v1beta1/proposals?proposal_status=2")?))
}

pub fn get_proposal_by_page_count(page: i64, count: i64) -> Result<PaginatedCosmosProposal, ExpectedError> {
  let url = format!("/cosmos/gov/v1beta1/proposals?pagination.offset={}&pagination.limit={}&pagination.count_total=true", (page - 1) * count, count);
  Ok(PaginatedCosmosProposal::new(request_to_node(url.as_str())?, page, count))
}

pub fn get_proposal_by_id(id: i64) -> Result<CosmosProposal, ExpectedError> {
  let url = format!("/cosmos/gov/v1beta1/proposals/{}", id);
  let response = request_to_node(url.as_str())?;
  let proposal = response.get("proposal");
  if proposal.is_none() {
    return Err(ExpectedError::RequestError(format!("proposal does not exist! proposal_id={}", id)));
  }
  Ok(CosmosProposal::new(proposal.unwrap().clone()))
}

pub fn get_validator_by_page_count(page: i64, count: i64) -> Result<PaginatedCosmosValidator, ExpectedError> {
  let url = format!("/cosmos/staking/v1beta1/validators?status=BOND_STATUS_BONDED&pagination.offset={}&pagination.limit={}&pagination.count_total=true", (page - 1) * count, count);
  Ok(PaginatedCosmosValidator::new(request_to_node(url.as_str())?, page, count))
}

pub fn get_validator_by_address(address: String) -> Result<CosmosValidator, ExpectedError> {
  let url = format!("/cosmos/staking/v1beta1/validators/{}", address);
  let response = request_to_node(url.as_str())?;
  let validator = response.get("validator");
  if validator.is_none() {
    return Err(ExpectedError::RequestError(format!("validator does not exist! validator_address={}", address)));
  }
  Ok(CosmosValidator::new(validator.unwrap().clone()))
}

pub fn request_to_node(api: &str) -> Result<Map<String, Value>, ExpectedError> {
  let node_endpoint = env::var("NODE_ENDPOINT");
  let res_res = reqwest::blocking::get(format!("{}/{}", node_endpoint.unwrap(), api));
  if res_res.is_err() {
    return Err(ExpectedError::RequestError(format!("request {} is failed", api)));
  }
  let res = res_res.unwrap();
  if !res.status().is_success() {
    return Err(ExpectedError::RequestError(format!("request {} is failed", api)));
  }
  let res_data = res.text();
  if res_data.is_err() {
    return Err(ExpectedError::RequestError(format!("parsing {} data is failed", api)));
  }
  let res_body = serde_json::from_str::<Map<String, Value>>(res_data.unwrap().as_str());
  if res_body.is_err() {
    return Err(ExpectedError::RequestError(format!("parsing {} body is failed", api)));
  }
  Ok(res_body.unwrap())
}
