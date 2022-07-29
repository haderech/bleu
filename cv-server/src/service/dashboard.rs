use paperclip::actix::api_v2_operation;
use paperclip::actix::web::Json;

use crate::error::error::ExpectedError;
use crate::model::dashboard::{CommunityPool, Inflation, NumOfVotingProposals, StakingPool};
use crate::service::node;

#[api_v2_operation(tags(Dashboard))]
pub async fn get_inflation() -> Result<Json<Inflation>, ExpectedError> {
  Ok(Json(node::get_inflation()?))
}

#[api_v2_operation(tags(Dashboard))]
pub async fn get_staking_pool() -> Result<Json<StakingPool>, ExpectedError> {
  Ok(Json(node::get_staking_pool()?))
}

#[api_v2_operation(tags(Dashboard))]
pub async fn get_community_pool() -> Result<Json<CommunityPool>, ExpectedError> {
  Ok(Json(node::get_community_pool()?))
}

#[api_v2_operation(tags(Dashboard))]
pub async fn get_num_of_voting_proposals() -> Result<Json<NumOfVotingProposals>, ExpectedError> {
  Ok(Json(node::get_num_of_voting_proposals()?))
}
