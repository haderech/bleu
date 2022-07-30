///cosmos/staking/v1beta1/validators?status=BOND_STATUS_BONDED&pagination.offset=0
use paperclip::actix::{api_v2_operation, web};
use paperclip::actix::web::Json;

use crate::error::error::ExpectedError;
use crate::model::pagination::RequestPage;
use crate::model::validator::{CosmosValidator, PaginatedCosmosValidator};
use crate::service::node;

#[api_v2_operation(tags(Validator))]
pub fn get_validator_by_page_count(req_page: web::Query<RequestPage>) -> Result<Json<PaginatedCosmosValidator>, ExpectedError> {
  Ok(Json(node::get_validator_by_page_count(req_page.page, req_page.count)?))
}

#[api_v2_operation(tags(Validator))]
pub fn get_validator_by_address(path_params: web::Path<String>) -> Result<Json<CosmosValidator>, ExpectedError> {
  let address = path_params.into_inner();
  Ok(Json(node::get_validator_by_address(address)?))
}
