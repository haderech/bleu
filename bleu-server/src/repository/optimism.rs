pub mod batch {
    use actix_web::web;
    use diesel::prelude::*;
    use diesel::RunQueryDsl;

    use crate::error::error::ExpectedError;
    use crate::model::optimism::{OptimismBatch, OptimismBatchSummary};
    use crate::Pool;
    use crate::schema::optimism::optimism_batches::dsl::*;

    pub fn find_latest_batch_summary(pool: web::Data<Pool>) -> Result<Vec<OptimismBatchSummary>, ExpectedError> {
        let conn = pool.get()?;
        let batch_summary = optimism_batches.select((batch_index, l1_tx_hash, batch_size, timestamp))
            .order(optimism_batches_id.desc())
            .limit(10)
            .load::<OptimismBatchSummary>(&conn)?;
        Ok(batch_summary)
    }

    pub fn find_batch_by_index(pool: web::Data<Pool>, index: i32) -> Result<OptimismBatch, ExpectedError> {
        let conn = pool.get()?;
        let result = optimism_batches.filter(batch_index.eq(index)).first::<OptimismBatch>(&conn);
        match result {
            Ok(optimism_batch) => Ok(optimism_batch),
            Err(_) => Err(ExpectedError::NotFoundError(format!("optimism batch does not exist! batch_index={}", index)))
        }
    }
}

pub mod tx {
    use actix_web::web;
    use diesel::prelude::*;
    use diesel::RunQueryDsl;

    use crate::error::error::ExpectedError;
    use crate::model::optimism::{OptimismTx, OptimismTxSummary};
    use crate::Pool;
    use crate::schema::optimism::optimism_txs::dsl::*;

    pub fn find_latest_tx_summary(pool: web::Data<Pool>) -> Result<Vec<OptimismTxSummary>, ExpectedError> {
        let conn = pool.get()?;
        let tx_summary = optimism_txs.select((tx_hash, from_address, to_address, value, timestamp))
            .order(optimism_txs_id.desc())
            .limit(10)
            .load::<OptimismTxSummary>(&conn)?;
        Ok(tx_summary)
    }

    pub fn find_tx_by_hash(pool: web::Data<Pool>, hash: String) -> Result<OptimismTx, ExpectedError> {
        let conn = pool.get()?;
        let result = optimism_txs.filter(tx_hash.eq(hash.clone())).first::<OptimismTx>(&conn);
        match result {
            Ok(optimism_tx) => Ok(optimism_tx),
            Err(_) => Err(ExpectedError::NotFoundError(format!("optimism tx does not exist! tx_hash={}", hash)))
        }
    }
}

pub mod l1_to_l2_tx {
    use actix_web::web;
    use diesel::prelude::*;
    use diesel::RunQueryDsl;

    use crate::error::error::ExpectedError;
    use crate::model::optimism::OptimismL1ToL2Tx;
    use crate::Pool;
    use crate::schema::optimism::optimism_l1_to_l2_txs::dsl::*;

    pub fn find_latest_l1_to_l2_tx(pool: web::Data<Pool>) -> Result<Vec<OptimismL1ToL2Tx>, ExpectedError> {
        let conn = pool.get()?;
        let tx_summary = optimism_l1_to_l2_txs.order(optimism_l1_to_l2_txs_id.desc())
            .limit(10)
            .load::<OptimismL1ToL2Tx>(&conn)?;
        Ok(tx_summary)
    }
}