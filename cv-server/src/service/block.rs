use paperclip::actix::{api_v2_operation, web};
use paperclip::actix::web::Json;

use crate::config::postgres::Pool;
use crate::error::error::ExpectedError;
use crate::model::block::*;
use crate::model::pagination::RequestPage;
use crate::repository::block;

#[api_v2_operation(tags(Block))]
pub async fn get_block_by_page_and_count(pool: web::Data<Pool>, req_page: web::Query<RequestPage>) -> Result<Json<PaginatedCosmosBlock>, ExpectedError> {
  Ok(Json(PaginatedCosmosBlock::new(block::cosmos_block::find_block_by_page_and_count(pool, req_page.page, req_page.count).await?)))
}

#[api_v2_operation(tags(Block))]
pub async fn get_block_by_height(pool: web::Data<Pool>, path_params: web::Path<i64>) -> Result<Json<CosmosBlock>, ExpectedError> {
  let height = path_params.into_inner();
  Ok(Json(block::cosmos_block::find_block_by_height(pool, height).await?))
}
