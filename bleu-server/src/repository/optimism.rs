use actix_web::web;
use diesel::prelude::*;
use diesel::RunQueryDsl;

use crate::error::error::ExpectedError;
use crate::model::optimism::{OptimismBatch, OptimismBatchSummary};
use crate::Pool;
use crate::schema::optimism::optimism_batches::dsl::*;

pub fn find_latest_batch_summary(pool: web::Data<Pool>) -> Result<Vec<OptimismBatchSummary>, ExpectedError> {
    let conn = pool.get()?;
    let batch_summary = optimism_batches.select((batch_index, l1_tx_hash, tx_size))
        .order(optimism_batches_id.desc())
        .limit(10)
        .load::<OptimismBatchSummary>(&conn)?;
    Ok(batch_summary)
}

pub fn find_batch_by_index(pool: web::Data<Pool>, index: u32) -> Result<OptimismBatch, ExpectedError> {
    let conn = pool.get()?;
    let index_str = format!("0x{:x}", index);
    let result = optimism_batches.filter(batch_index.eq(index_str)).first(&conn);
    match result {
        Ok(optimism_batch) => Ok(optimism_batch),
        Err(_) => Err(ExpectedError::NotFoundError(format!("optimism batch does not exist! batch_index={}", index)))
    }
}