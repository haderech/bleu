use actix_web::{get, HttpResponse, web};

use crate::error::error::ExpectedError;
use crate::Pool;
use crate::repository::optimism;

#[get("/optimism/batch/latest")]
pub async fn get_latest_batch_summary(pool: web::Data<Pool>) -> Result<HttpResponse, ExpectedError> {
    let batch_summary = optimism::find_latest_batch_summary(pool)?;
    Ok(HttpResponse::Ok().json(batch_summary))
}

#[get("/optimism/batch/{batch_index}")]
pub async fn get_batch_by_index(pool: web::Data<Pool>, batch_index: web::Path<(u32)>) -> Result<HttpResponse, ExpectedError> {
    let index = batch_index.into_inner();
    let batch = optimism::find_batch_by_index(pool, index)?;
    Ok(HttpResponse::Ok().json(batch))
}