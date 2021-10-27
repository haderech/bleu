pub mod tx_batch {
    use actix_web::web;
    use cached::proc_macro::cached;
    use diesel::prelude::*;
    use diesel::RunQueryDsl;

    use crate::error::error::ExpectedError;
    use crate::model::optimism::{OptimismTxBatch, OptimismTxBatchSummary};
    use crate::Pool;
    use crate::repository::pagination::{LoadPaginated, PaginatedRecord};
    use crate::schema::optimism::optimism_tx_batches;
    use crate::schema::optimism::optimism_tx_batches::columns::*;

    #[cached(time = 60, key = "bool", convert = r#"{ true }"#, result = true)]
    pub fn find_latest_batch_summary(pool: web::Data<Pool>) -> Result<Vec<OptimismTxBatchSummary>, ExpectedError> {
        let conn = pool.get()?;
        let batch_summary = optimism_tx_batches::table.select((batch_index, l1_tx_hash, batch_size, batch_timestamp))
            .order(optimism_tx_batches_id.desc())
            .limit(10)
            .load::<OptimismTxBatchSummary>(&conn)?;
        Ok(batch_summary)
    }

    pub fn find_batch_by_index(pool: web::Data<Pool>, index: i64) -> Result<OptimismTxBatch, ExpectedError> {
        let conn = pool.get()?;
        Ok(optimism_tx_batches::table.filter(batch_index.eq(index.to_string())).first::<OptimismTxBatch>(&conn)?)
    }

    pub fn find_batch_by_page_count(pool: web::Data<Pool>, page: i64, count: i64) -> Result<PaginatedRecord<OptimismTxBatch>, ExpectedError> {
        let conn = pool.get()?;
        let query = optimism_tx_batches::table.into_boxed();
        let query = query.order(optimism_tx_batches_id.desc());
        let paginated_batch = query.load_with_pagination(&conn, page, count)?;
        Ok(paginated_batch)
    }
}

pub mod tx {
    use actix_web::web;
    use cached::proc_macro::cached;
    use diesel::prelude::*;
    use diesel::RunQueryDsl;

    use crate::error::error::ExpectedError;
    use crate::model::optimism::{OptimismBlockTx, OptimismL1ToL2TxSummary, OptimismTxSummary};
    use crate::Pool;
    use crate::repository::pagination::{LoadPaginated, PaginatedRecord};
    use crate::schema::optimism::{optimism_block_txs, optimism_state_roots};
    use crate::schema::optimism::optimism_block_txs::columns::*;
    use crate::schema::optimism::optimism_txs;

    #[cached(time = 60, key = "bool", convert = r#"{ true }"#, result = true)]
    pub fn find_latest_tx_summary(pool: web::Data<Pool>) -> Result<Vec<OptimismTxSummary>, ExpectedError> {
        let conn = pool.get()?;
        let tx_summary = optimism_block_txs::table.select((hash, from_address, to_address, value, l1_timestamp))
            .order(optimism_block_txs_id.desc())
            .limit(10)
            .load::<OptimismTxSummary>(&conn)?;
        Ok(tx_summary)
    }

    pub fn find_tx_by_hash(pool: web::Data<Pool>, tx_hash: String) -> Result<OptimismBlockTx, ExpectedError> {
        let conn = pool.get()?;
        Ok(optimism_block_txs::table.filter(hash.eq(tx_hash.clone())).first::<OptimismBlockTx>(&conn)?)
    }

    pub fn find_tx_by_index(pool: web::Data<Pool>, batch_tx_index: i64) -> Result<OptimismBlockTx, ExpectedError> {
        let conn = pool.get()?;
        Ok(optimism_block_txs::table.filter(index.eq(batch_tx_index.to_string())).first::<OptimismBlockTx>(&conn)?)
    }

    pub fn find_tx_by_tx_batch_index_page_count(pool: web::Data<Pool>, tx_batch_index: i64, page: i64, count: i64) -> Result<PaginatedRecord<OptimismBlockTx>, ExpectedError> {
        let conn = pool.get()?;
        let query = optimism_block_txs::table.inner_join(optimism_txs::table.on(
            optimism_block_txs::index.eq(optimism_txs::index).and(optimism_txs::batch_index.eq(tx_batch_index.to_string()))
        )).order(optimism_block_txs_id.desc())
            .select(optimism_block_txs::all_columns);
        let paginated_tx = query.load_with_pagination(&conn, page, count)?;
        Ok(paginated_tx)
    }

    pub fn find_tx_by_state_batch_index_page_count(pool: web::Data<Pool>, state_batch_index: i64, page: i64, count: i64) -> Result<PaginatedRecord<OptimismBlockTx>, ExpectedError> {
        let conn = pool.get()?;
        let query = optimism_block_txs::table.inner_join(optimism_state_roots::table.on(
            optimism_block_txs::index.eq(optimism_state_roots::index).and(optimism_state_roots::batch_index.eq(state_batch_index.to_string()))
        )).order(optimism_block_txs_id.desc())
            .select(optimism_block_txs::all_columns);
        let paginated_tx = query.load_with_pagination(&conn, page, count)?;
        Ok(paginated_tx)
    }

    #[cached(time = 60, key = "bool", convert = r#"{ true }"#, result = true)]
    pub fn find_latest_l1_to_l2_tx_summary(pool: web::Data<Pool>) -> Result<Vec<OptimismL1ToL2TxSummary>, ExpectedError> {
        Ok(Vec::new())
    }

    // pub fn find_l1_to_l2_tx_by_page_count(pool: web::Data<Pool>, page: i64, count: i64) -> Result<PaginatedRecord<OptimismL1ToL2Tx>, ExpectedError> {
    // }
}

pub mod state_batch {
    use actix_web::web;
    use diesel::prelude::*;
    use diesel::RunQueryDsl;

    use crate::error::error::ExpectedError;
    use crate::model::optimism::OptimismStateBatch;
    use crate::Pool;
    use crate::repository::pagination::{LoadPaginated, PaginatedRecord};
    use crate::schema::optimism::optimism_state_batches;
    use crate::schema::optimism::optimism_state_batches::columns::*;

    pub fn find_batch_by_page_count(pool: web::Data<Pool>, page: i64, count: i64) -> Result<PaginatedRecord<OptimismStateBatch>, ExpectedError> {
        let conn = pool.get()?;
        let query = optimism_state_batches::table.into_boxed();
        let query = query.order(optimism_state_batches_id.desc());
        let paginated_batch = query.load_with_pagination(&conn, page, count)?;
        Ok(paginated_batch)
    }

    pub fn find_batch_by_index(pool: web::Data<Pool>, index: i64) -> Result<OptimismStateBatch, ExpectedError> {
        let conn = pool.get()?;
        Ok(optimism_state_batches::table.filter(batch_index.eq(index.to_string())).first::<OptimismStateBatch>(&conn)?)
    }
}