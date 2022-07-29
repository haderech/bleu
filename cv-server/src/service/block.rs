use paperclip::actix::{api_v2_operation, web};
use paperclip::actix::web::Json;
use serde_json::from_str;

use crate::config::postgres::Pool;
use crate::error::error::ExpectedError;
use crate::model::block::*;
use crate::model::pagination::RequestPage;
use crate::repository::block;
use crate::repository::block::cosmos_block::{find_block_summary, find_latest_block};

#[api_v2_operation(tags(Block))]
pub async fn get_block_by_page_and_count(pool: web::Data<Pool>, req_page: web::Query<RequestPage>) -> Result<Json<PaginatedCosmosBlock>, ExpectedError> {
  Ok(Json(PaginatedCosmosBlock::new(block::cosmos_block::find_block_by_page_and_count(pool, req_page.page, req_page.count).await?)))
}

#[api_v2_operation(tags(Block))]
pub async fn get_block_by_height(pool: web::Data<Pool>, path_params: web::Path<String>) -> Result<Json<CosmosBlock>, ExpectedError> {
  let height = path_params.into_inner();
  if let Ok(block_height) = from_str::<i64>(height.as_str()) {
    Ok(Json(block::cosmos_block::find_block_by_height(pool, block_height).await?))
  } else {
    Ok(Json(find_latest_block(pool).await?))
  }
}

#[api_v2_operation(tags(Block))]
pub async fn get_block_summary(pool: web::Data<Pool>) -> Result<Json<Vec<CosmosBlock>>, ExpectedError> {
  Ok(Json(find_block_summary(pool).await?))
}
