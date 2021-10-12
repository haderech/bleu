use actix_web::{get, HttpResponse, web};

use crate::error::error::ExpectedError;
use crate::Pool;
use crate::repository::optimism;

#[get("/optimism/batch/latest")]
pub async fn get_latest_batch_summary(pool: web::Data<Pool>) -> Result<HttpResponse, ExpectedError> {
    let batch_summary = optimism::batch::find_latest_batch_summary(pool)?;
    Ok(HttpResponse::Ok().json(batch_summary))
}

#[get("/optimism/batch/index/{batch_index}")]
pub async fn get_batch_by_index(pool: web::Data<Pool>, batch_index: web::Path<(u32)>) -> Result<HttpResponse, ExpectedError> {
    let index = batch_index.into_inner();
    let batch = optimism::batch::find_batch_by_index(pool, index)?;
    Ok(HttpResponse::Ok().json(batch))
}

#[get("/optimism/tx/latest")]
pub async fn get_latest_tx_summary(pool: web::Data<Pool>) -> Result<HttpResponse, ExpectedError> {
    let tx_summary = optimism::tx::find_latest_tx_summary(pool)?;
    Ok(HttpResponse::Ok().json(tx_summary))
}

#[get("/optimism/tx/hash/{tx_hash}")]
pub async fn get_tx_by_hash(pool: web::Data<Pool>, tx_hash: web::Path<(String)>) -> Result<HttpResponse, ExpectedError> {
    let hash = tx_hash.into_inner();
    let tx = optimism::tx::find_tx_by_hash(pool, hash)?;
    Ok(HttpResponse::Ok().json(tx))
}

#[get("/optimism/tx/l1tol2/latest")]
pub async fn get_latest_l1_to_l2_tx(pool: web::Data<Pool>) -> Result<HttpResponse, ExpectedError> {
    let tx = optimism::l1_to_l2_tx::find_latest_l1_to_l2_tx(pool)?;
    Ok(HttpResponse::Ok().json(tx))
}