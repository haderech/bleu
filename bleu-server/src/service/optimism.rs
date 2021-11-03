use paperclip::actix::{api_v2_operation, web};
use paperclip::actix::web::Json;

use crate::config::postgres::Pool;
use crate::error::error::ExpectedError;
use crate::model::optimism::*;
use crate::repository::optimism;

#[api_v2_operation(tags(TxBatch))]
pub async fn get_latest_tx_batch_summary(pool: web::Data<Pool>) -> Result<Json<Vec<OptimismTxBatchSummary>>, ExpectedError> {
    Ok(Json(optimism::tx_batch::find_latest_batch_summary(pool).await?))
}

#[api_v2_operation(tags(TxBatch))]
pub async fn get_tx_batch_by_index(pool: web::Data<Pool>, path_params: web::Path<i64>) -> Result<Json<OptimismTxBatch>, ExpectedError> {
    let index = path_params.into_inner();
    Ok(Json(optimism::tx_batch::find_batch_by_index(pool, index).await?))
}

#[api_v2_operation(tags(TxBatch))]
pub async fn get_paginated_tx_batch(pool: web::Data<Pool>, path_params: web::Path<(i64, i64)>) -> Result<Json<PaginatedOptimismTxBatch>, ExpectedError> {
    let (page, count) = path_params.into_inner();
    Ok(Json(PaginatedOptimismTxBatch::new(optimism::tx_batch::find_batch_by_page_count(pool, page, count).await?)))
}

#[api_v2_operation(tags(Tx))]
pub async fn get_latest_tx_summary(pool: web::Data<Pool>) -> Result<Json<Vec<OptimismTxSummary>>, ExpectedError> {
    Ok(Json(optimism::tx::find_latest_tx_summary(pool).await?))
}

#[api_v2_operation(tags(Tx))]
pub async fn get_tx_by_hash(pool: web::Data<Pool>, path_params: web::Path<String>) -> Result<Json<OptimismBlockTx>, ExpectedError> {
    let hash = path_params.into_inner();
    Ok(Json(optimism::tx::find_tx_by_hash(pool, hash).await?))
}

#[api_v2_operation(tags(Tx))]
pub async fn get_tx_by_index(pool: web::Data<Pool>, path_params: web::Path<i64>) -> Result<Json<OptimismBlockTx>, ExpectedError> {
    let batch_tx_index = path_params.into_inner();
    Ok(Json(optimism::tx::find_tx_by_index(pool, batch_tx_index).await?))
}

#[api_v2_operation(tags(Tx))]
pub async fn get_paginated_tx_by_tx_batch_index(pool: web::Data<Pool>, path_params: web::Path<(i64, i64, i64)>) -> Result<Json<PaginatedOptimismBlockTx>, ExpectedError> {
    let (tx_batch_index, page, count) = path_params.into_inner();
    Ok(Json(PaginatedOptimismBlockTx::new(optimism::tx::find_tx_by_tx_batch_index_page_count(pool, tx_batch_index, page, count).await?)))
}

#[api_v2_operation(tags(Tx))]
pub async fn get_paginated_tx_by_state_batch_index(pool: web::Data<Pool>, path_params: web::Path<(i64, i64, i64)>) -> Result<Json<PaginatedOptimismBlockTx>, ExpectedError> {
    let (state_batch_index, page, count) = path_params.into_inner();
    Ok(Json(PaginatedOptimismBlockTx::new(optimism::tx::find_tx_by_state_batch_index_page_count(pool, state_batch_index, page, count).await?)))
}

#[api_v2_operation(tags(StateRootBatch))]
pub async fn get_paginated_state_batch(pool: web::Data<Pool>, path_params: web::Path<(i64, i64)>) -> Result<Json<PaginatedOptimismStateBatch>, ExpectedError> {
    let (page, count) = path_params.into_inner();
    Ok(Json(PaginatedOptimismStateBatch::new(optimism::state_batch::find_batch_by_page_count(pool, page, count).await?)))
}

#[api_v2_operation(tags(StateRootBatch))]
pub async fn get_state_batch_by_index(pool: web::Data<Pool>, path_params: web::Path<i64>) -> Result<Json<OptimismStateBatch>, ExpectedError> {
    let index = path_params.into_inner();
    Ok(Json(optimism::state_batch::find_batch_by_index(pool, index).await?))
}

#[api_v2_operation(tags(L1ToL2))]
pub async fn get_latest_l1_to_l2_tx_summary(pool: web::Data<Pool>) -> Result<Json<Vec<OptimismL1ToL2TxSummary>>, ExpectedError> {
    Ok(Json(optimism::l1_to_l2::find_latest_l1_to_l2_tx_summary(pool).await?))
}

#[api_v2_operation(tags(L1ToL2))]
pub async fn get_l1_to_l2_tx_by_page_count(pool: web::Data<Pool>, path_params: web::Path<(i64, i64)>) -> Result<Json<PaginatedOptimismL1ToL2Tx>, ExpectedError> {
    let (page, count) = path_params.into_inner();
    Ok(Json(PaginatedOptimismL1ToL2Tx::new(optimism::l1_to_l2::find_l1_to_l2_tx_by_page_count(pool, page, count).await?)))
}

#[api_v2_operation(tags(TxLogs))]
pub async fn get_tx_logs_by_hash(pool: web::Data<Pool>, path_params: web::Path<String>) -> Result<Json<Vec<OptimismTxLog>>, ExpectedError> {
    let hash = path_params.into_inner();
    let entity_vec = optimism::tx_logs::find_tx_logs_by_hash(pool, hash).await?;
    Ok(Json(entity_vec.into_iter().map(|e| { OptimismTxLog::from(e) }).collect::<Vec<OptimismTxLog>>()))
}

#[api_v2_operation(tags(BoardSummary))]
pub async fn get_board_summary(pool: web::Data<Pool>) -> Result<Json<BoardSummary>, ExpectedError> {
    let board_summary = optimism::summary::find_board_summary(pool).await?;
    Ok(Json(board_summary))
}