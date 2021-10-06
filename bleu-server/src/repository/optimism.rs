use actix_web::web;
use diesel::prelude::*;
use diesel::RunQueryDsl;

use crate::error::error::ExpectedError;
use crate::model::optimism::OptimismBatch;
use crate::Pool;
use crate::schema::optimism::optimism_batches::dsl::*;

pub fn find_latest_batches(pool: web::Data<Pool>) -> Result<Vec<OptimismBatch>, ExpectedError> {
    let conn = pool.get()?;
    let latest_batches = optimism_batches.order(optimism_batches_id.desc()).limit(10).load::<OptimismBatch>(&conn)?;
    Ok(latest_batches)
}