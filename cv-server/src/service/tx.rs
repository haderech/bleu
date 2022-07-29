use paperclip::actix::{api_v2_operation, web};
use paperclip::actix::web::Json;

use crate::config::postgres::Pool;
use crate::error::error::ExpectedError;
use crate::model::pagination::RequestPage;
use crate::model::tx::*;
use crate::repository::tx;

#[api_v2_operation(tags(Tx))]
pub async fn get_tx_by_page_and_count(pool: web::Data<Pool>, req_page: web::Query<RequestPage>) -> Result<Json<PaginatedCosmosTx>, ExpectedError> {
  Ok(Json(PaginatedCosmosTx::new(tx::cosmos_tx::find_tx_by_page_and_count(pool, req_page.page, req_page.count).await?)))
}

#[api_v2_operation(tags(Tx))]
pub async fn get_tx_by_height_and_page_and_count(pool: web::Data<Pool>, path_params: web::Path<i64>, req_page: web::Query<RequestPage>) -> Result<Json<PaginatedCosmosTx>, ExpectedError> {
  let height = path_params.into_inner();
  Ok(Json(PaginatedCosmosTx::new(tx::cosmos_tx::find_tx_by_height_and_page_and_count(pool, height, req_page.page, req_page.count).await?)))
}

#[api_v2_operation(tags(Tx))]
pub async fn get_tx_by_hash(pool: web::Data<Pool>, path_params: web::Path<String>) -> Result<Json<CosmosTx>, ExpectedError> {
  let hash = path_params.into_inner();
  Ok(Json(tx::cosmos_tx::find_tx_by_hash(pool, hash).await?))
}

#[api_v2_operation(tags(Tx))]
pub async fn get_tx_summary(pool: web::Data<Pool>) -> Result<Json<Vec<CosmosTx>>, ExpectedError> {
  Ok(Json(tx::cosmos_tx::find_tx_summary(pool).await?))
}
