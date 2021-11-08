pub mod tx_batch {
    use actix_web::web;
    use cached::proc_macro::cached;
    use diesel::prelude::*;
    use diesel::RunQueryDsl;

    use crate::config::postgres::{PgConn, Pool};
    use crate::error::error::ExpectedError;
    use crate::model::optimism::{OptimismTxBatch, OptimismTxBatchSummary};
    use crate::repository::pagination::{LoadPaginated, PaginatedRecord};
    use crate::schema::optimism::optimism_tx_batches;
    use crate::schema::optimism::optimism_tx_batches::columns::*;

    #[cached(time = 60, key = "bool", convert = r#"{ true }"#, result = true)]
    pub async fn find_latest_batch_summary(pool: web::Data<Pool>) -> Result<Vec<OptimismTxBatchSummary>, ExpectedError> {
        let conn = pool.get()?;
        let batch_summary = web::block(move || {
            optimism_tx_batches::table.select((batch_index, l1_tx_hash, batch_size, batch_timestamp))
                .order(optimism_tx_batches_id.desc())
                .limit(10)
                .load::<OptimismTxBatchSummary>(&conn)
        }).await?;
        Ok(batch_summary)
    }

    pub async fn find_batch_by_index(pool: web::Data<Pool>, index: i64) -> Result<OptimismTxBatch, ExpectedError> {
        let conn = pool.get()?;
        let batch_summary = web::block(move || {
            optimism_tx_batches::table.filter(batch_index.eq(index.to_string()))
                .first::<OptimismTxBatch>(&conn)
        }).await?;
        Ok(batch_summary)
    }

    pub async fn find_batch_by_page_count(pool: web::Data<Pool>, page: i64, count: i64) -> Result<PaginatedRecord<OptimismTxBatch>, ExpectedError> {
        let conn = pool.get()?;
        let paginated_batch = web::block(move || {
            optimism_tx_batches::table.into_boxed()
                .order(optimism_tx_batches_id.desc())
                .load_with_pagination(&conn, page, count)
        }).await?;
        Ok(paginated_batch)
    }

    pub async fn find_latest_tx_batch(conn: PgConn) -> Result<OptimismTxBatch, ExpectedError> {
        let latest_tx_batch = web::block(move || {
            optimism_tx_batches::table.into_boxed()
                .order(optimism_tx_batches_id.desc())
                .first::<OptimismTxBatch>(&conn)
        }).await?;
        Ok(latest_tx_batch)
    }
}

pub mod tx {
    use actix_web::web;
    use cached::proc_macro::cached;
    use diesel::prelude::*;
    use diesel::RunQueryDsl;

    use crate::config::postgres::{PgConn, Pool};
    use crate::error::error::ExpectedError;
    use crate::model::optimism::{OptimismBlockTx, OptimismTxSummary};
    use crate::repository::pagination::{LoadPaginated, PaginatedRecord};
    use crate::schema::optimism::{optimism_block_txs, optimism_state_roots};
    use crate::schema::optimism::optimism_block_txs::columns::*;
    use crate::schema::optimism::optimism_txs;

    #[cached(time = 60, key = "bool", convert = r#"{ true }"#, result = true)]
    pub async fn find_latest_tx_summary(pool: web::Data<Pool>) -> Result<Vec<OptimismTxSummary>, ExpectedError> {
        let conn = pool.get()?;
        let tx_summary = web::block(move || {
            optimism_block_txs::table.select((hash, from_address, to_address, value, l1_timestamp))
                .order(optimism_block_txs_id.desc())
                .limit(10)
                .load::<OptimismTxSummary>(&conn)
        }).await?;
        Ok(tx_summary)
    }

    pub async fn find_tx_by_hash(pool: web::Data<Pool>, tx_hash: String) -> Result<OptimismBlockTx, ExpectedError> {
        let conn = pool.get()?;
        let tx = web::block(move || {
            optimism_block_txs::table.filter(hash.eq(tx_hash.clone()))
                .first::<OptimismBlockTx>(&conn)
        }).await?;
        Ok(tx)
    }

    pub async fn find_tx_by_index(pool: web::Data<Pool>, batch_tx_index: i64) -> Result<OptimismBlockTx, ExpectedError> {
        let conn = pool.get()?;
        let tx = web::block(move || {
            optimism_block_txs::table.filter(index.eq(batch_tx_index.to_string()))
                .first::<OptimismBlockTx>(&conn)
        }).await?;
        Ok(tx)
    }

    pub async fn find_tx_by_tx_batch_index_page_count(pool: web::Data<Pool>, tx_batch_index: i64, page: i64, count: i64) -> Result<PaginatedRecord<OptimismBlockTx>, ExpectedError> {
        let conn = pool.get()?;
        let paginated_tx = web::block(move || {
            optimism_block_txs::table.inner_join(optimism_txs::table.on(
                optimism_block_txs::index.eq(optimism_txs::index).and(optimism_txs::batch_index.eq(tx_batch_index.to_string()))
            )).order(optimism_block_txs_id.desc())
                .select(optimism_block_txs::all_columns)
                .load_with_pagination(&conn, page, count)
        }).await?;
        Ok(paginated_tx)
    }

    pub async fn find_tx_by_state_batch_index_page_count(pool: web::Data<Pool>, state_batch_index: i64, page: i64, count: i64) -> Result<PaginatedRecord<OptimismBlockTx>, ExpectedError> {
        let conn = pool.get()?;
        let paginated_tx = web::block(move || {
            optimism_block_txs::table.inner_join(optimism_state_roots::table.on(
                optimism_block_txs::index.eq(optimism_state_roots::index).and(optimism_state_roots::batch_index.eq(state_batch_index.to_string()))
            )).order(optimism_block_txs_id.desc())
                .select(optimism_block_txs::all_columns)
                .load_with_pagination(&conn, page, count)
        }).await?;
        Ok(paginated_tx)
    }

    pub async fn find_tx_count(conn: PgConn) -> Result<i64, ExpectedError> {
        let tx_count = web::block(move || {
            optimism_block_txs::table.count().first(&conn)
        }).await?;
        Ok(tx_count)
    }

    pub async fn find_tx_by_address_page_count(pool: web::Data<Pool>, address: String, page: i64, count: i64) -> Result<PaginatedRecord<OptimismBlockTx>, ExpectedError> {
        let conn = pool.get()?;
        let paginated_tx = web::block(move || {
            optimism_block_txs::table.filter(from_address.eq(address.clone()).or(to_address.eq(address)))
                .order(optimism_block_txs_id.desc())
                .load_with_pagination(&conn, page, count)
        }).await?;
        Ok(paginated_tx)
    }
}

pub mod state_batch {
    use actix_web::web;
    use diesel::prelude::*;
    use diesel::RunQueryDsl;

    use crate::config::postgres::{PgConn, Pool};
    use crate::error::error::ExpectedError;
    use crate::model::optimism::OptimismStateBatch;
    use crate::repository::pagination::{LoadPaginated, PaginatedRecord};
    use crate::schema::optimism::optimism_state_batches;
    use crate::schema::optimism::optimism_state_batches::columns::*;

    pub async fn find_batch_by_page_count(pool: web::Data<Pool>, page: i64, count: i64) -> Result<PaginatedRecord<OptimismStateBatch>, ExpectedError> {
        let conn = pool.get()?;
        let paginated_batch = web::block(move || {
            optimism_state_batches::table.into_boxed()
                .order(optimism_state_batches_id.desc())
                .load_with_pagination(&conn, page, count)
        }).await?;
        Ok(paginated_batch)
    }

    pub async fn find_batch_by_index(pool: web::Data<Pool>, index: i64) -> Result<OptimismStateBatch, ExpectedError> {
        let conn = pool.get()?;
        let state_batch = web::block(move || {
            optimism_state_batches::table.filter(batch_index.eq(index.to_string()))
                .first::<OptimismStateBatch>(&conn)
        }).await?;
        Ok(state_batch)
    }

    pub async fn find_latest_state_batch(conn: PgConn) -> Result<OptimismStateBatch, ExpectedError> {
        let latest_state_batch = web::block(move || {
            optimism_state_batches::table.into_boxed()
                .order(optimism_state_batches_id.desc())
                .first::<OptimismStateBatch>(&conn)
        }).await?;
        Ok(latest_state_batch)
    }
}

pub mod l1_to_l2 {
    use actix_web::web;
    use cached::proc_macro::cached;
    use diesel::prelude::*;
    use diesel::RunQueryDsl;

    use crate::config::postgres::Pool;
    use crate::error::error::ExpectedError;
    use crate::model::optimism::{OptimismL1ToL2Tx, OptimismL1ToL2TxSummary};
    use crate::repository::pagination::{LoadPaginated, PaginatedRecord};
    use crate::schema::ethereum::ethereum_tx_logs;
    use crate::schema::optimism::optimism_block_txs;

    #[cached(time = 60, key = "bool", convert = r#"{ true }"#, result = true)]
    pub async fn find_latest_l1_to_l2_tx_summary(pool: web::Data<Pool>) -> Result<Vec<OptimismL1ToL2TxSummary>, ExpectedError> {
        let conn = pool.get()?;
        let l1_to_l2_tx_summary = web::block(move || {
            optimism_block_txs::table.inner_join(
                ethereum_tx_logs::table.on(optimism_block_txs::queue_index.eq(ethereum_tx_logs::queue_index))
            )
                .filter(optimism_block_txs::queue_origin.eq("l1"))
                .order(optimism_block_txs::optimism_block_txs_id.desc())
                .select((optimism_block_txs::l1_block_number, ethereum_tx_logs::tx_hash, optimism_block_txs::hash))
                .limit(10)
                .load::<OptimismL1ToL2TxSummary>(&conn)
        }).await?;
        Ok(l1_to_l2_tx_summary)
    }

    pub async fn find_l1_to_l2_tx_by_page_count(pool: web::Data<Pool>, page: i64, count: i64) -> Result<PaginatedRecord<OptimismL1ToL2Tx>, ExpectedError> {
        let conn = pool.get()?;
        let paginated_l1_to_l2_tx = web::block(move || {
            optimism_block_txs::table.inner_join(
                ethereum_tx_logs::table.on(optimism_block_txs::queue_index.eq(ethereum_tx_logs::queue_index))
            )
                .filter(optimism_block_txs::queue_origin.eq("l1"))
                .order(optimism_block_txs::optimism_block_txs_id.desc())
                .select((
                    optimism_block_txs::l1_block_number,
                    optimism_block_txs::queue_index,
                    optimism_block_txs::hash,
                    optimism_block_txs::l1_timestamp,
                    ethereum_tx_logs::tx_hash,
                    optimism_block_txs::l1_tx_origin,
                    optimism_block_txs::gas
                ))
                .load_with_pagination(&conn, page, count)
        }).await?;
        Ok(paginated_l1_to_l2_tx)
    }
}

pub mod tx_logs {
    use actix_web::web;
    use diesel::prelude::*;
    use diesel::RunQueryDsl;

    use crate::config::postgres::Pool;
    use crate::error::error::ExpectedError;
    use crate::model::optimism::OptimismTxReceiptLog;
    use crate::schema::optimism::optimism_tx_receipt_logs;
    use crate::schema::optimism::optimism_tx_receipt_logs::columns::*;

    pub async fn find_tx_logs_by_hash(pool: web::Data<Pool>, hash: String) -> Result<Vec<OptimismTxReceiptLog>, ExpectedError> {
        let conn = pool.get()?;
        let tx_logs = web::block(move || {
            optimism_tx_receipt_logs::table.filter(tx_hash.eq(hash))
                .load::<OptimismTxReceiptLog>(&conn)
        }).await?;
        Ok(tx_logs)
    }
}

pub mod summary {
    use actix_web::web;
    use cached::proc_macro::cached;

    use crate::config::postgres::Pool;
    use crate::error::error::ExpectedError;
    use crate::model::optimism::BoardSummary;
    use crate::repository::optimism::state_batch::find_latest_state_batch;
    use crate::repository::optimism::tx::find_tx_count;
    use crate::repository::optimism::tx_batch::find_latest_tx_batch;

    #[cached(time = 60, key = "bool", convert = r#"{ true }"#, result = true)]
    pub async fn find_board_summary(pool: web::Data<Pool>) -> Result<BoardSummary, ExpectedError> {
        let conn = pool.get()?;
        let latest_tx_batch = find_latest_tx_batch(conn).await?;
        let conn = pool.get()?;
        let latest_state_batch = find_latest_state_batch(conn).await?;
        let conn = pool.get()?;
        let tx_count = find_tx_count(conn).await?;

        let board_summary = BoardSummary::new(
            latest_tx_batch.get_batch_index(),
            latest_state_batch.get_batch_index(),
            tx_count,
        );
        Ok(board_summary)
    }
}