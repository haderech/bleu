use paperclip::actix::{api_v2_operation, web};
use paperclip::actix::web::Json;

use crate::error::error::ExpectedError;
use crate::model::pagination::RequestPage;
use crate::model::proposal::{CosmosProposal, PaginatedCosmosProposal};
use crate::service::node;

#[api_v2_operation(tags(Proposal))]
pub fn get_proposal_by_page_count(req_page: web::Query<RequestPage>) -> Result<Json<PaginatedCosmosProposal>, ExpectedError> {
  Ok(Json(node::get_proposal_by_page_count(req_page.page, req_page.count)?))
}

#[api_v2_operation(tags(Proposal))]
pub fn get_proposal_by_id(path_params: web::Path<i64>) -> Result<Json<CosmosProposal>, ExpectedError> {
  let id = path_params.into_inner();
  Ok(Json(node::get_proposal_by_id(id)?))
}
