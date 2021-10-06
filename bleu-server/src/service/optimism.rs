use actix_web::{get, HttpResponse, web};

use crate::error::error::ExpectedError;
use crate::Pool;
use crate::repository::optimism;

#[get("/optimism/batch/latest")]
pub async fn get_latest_batches(pool: web::Data<Pool>) -> Result<HttpResponse, ExpectedError> {
    let latest_batches = optimism::find_latest_batches(pool)?;
    Ok(HttpResponse::Ok().json(latest_batches))
}