use paperclip::actix::{api_v2_operation, web};
use paperclip::actix::web::Json;

use crate::error::error::ExpectedError;
use crate::model::optimism::{OptimismBlockTx, OptimismL1ToL2Tx, OptimismStateBatch, OptimismTxBatch, OptimismTxBatchSummary, OptimismTxSummary, PaginatedOptimismBlockTx, PaginatedOptimismStateBatch, PaginatedOptimismTxBatch};
use crate::Pool;
use crate::repository::optimism;

#[api_v2_operation]
pub async fn get_latest_tx_batch_summary(pool: web::Data<Pool>) -> Result<Json<Vec<OptimismTxBatchSummary>>, ExpectedError> {
    Ok(Json(optimism::tx_batch::find_latest_batch_summary(pool)?))
}

#[api_v2_operation]
pub async fn get_tx_batch_by_index(pool: web::Data<Pool>, path_params: web::Path<i64>) -> Result<Json<OptimismTxBatch>, ExpectedError> {
    let index = path_params.into_inner();
    Ok(Json(optimism::tx_batch::find_batch_by_index(pool, index)?))
}

#[api_v2_operation]
pub async fn get_paginated_tx_batch(pool: web::Data<Pool>, path_params: web::Path<(i64, i64)>) -> Result<Json<PaginatedOptimismTxBatch>, ExpectedError> {
    let (page, count) = path_params.into_inner();
    Ok(Json(PaginatedOptimismTxBatch::new(optimism::tx_batch::find_batch_by_page_count(pool, page, count)?)))
}

#[api_v2_operation]
pub async fn get_latest_tx_summary(pool: web::Data<Pool>) -> Result<Json<Vec<OptimismTxSummary>>, ExpectedError> {
    Ok(Json(optimism::tx::find_latest_tx_summary(pool)?))
}

#[api_v2_operation]
pub async fn get_tx_by_hash(pool: web::Data<Pool>, path_params: web::Path<String>) -> Result<Json<OptimismBlockTx>, ExpectedError> {
    let hash = path_params.into_inner();
    Ok(Json(optimism::tx::find_tx_by_hash(pool, hash)?))
}

#[api_v2_operation]
pub async fn get_paginated_tx(pool: web::Data<Pool>, path_params: web::Path<(i64, i64, i64)>) -> Result<Json<PaginatedOptimismBlockTx>, ExpectedError> {
    let (batch_index, page, count) = path_params.into_inner();
    Ok(Json(PaginatedOptimismBlockTx::new(optimism::tx::find_tx_by_index_page_count(pool, batch_index, page, count)?)))
}

#[api_v2_operation]
pub async fn get_paginated_state_batch(pool: web::Data<Pool>, path_params: web::Path<(i64, i64)>) -> Result<Json<PaginatedOptimismStateBatch>, ExpectedError> {
    let (page, count) = path_params.into_inner();
    Ok(Json(PaginatedOptimismStateBatch::new(optimism::state_batch::find_batch_by_page_count(pool, page, count)?)))
}

#[api_v2_operation]
pub async fn get_state_batch_by_index(pool: web::Data<Pool>, path_params: web::Path<i64>) -> Result<Json<OptimismStateBatch>, ExpectedError> {
    let index = path_params.into_inner();
    Ok(Json(optimism::state_batch::find_batch_by_index(pool, index)?))
}

#[api_v2_operation]
pub async fn get_latest_l1_to_l2_tx(pool: web::Data<Pool>) -> Result<Json<Vec<OptimismL1ToL2Tx>>, ExpectedError> {
    Ok(Json(optimism::l1_to_l2_tx::find_latest_l1_to_l2_tx(pool)?))
}