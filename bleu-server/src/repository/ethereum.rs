pub mod ethereum_log {
    use actix_web::web;
    use diesel::prelude::*;

    use crate::config::postgres::PgConn;
    use crate::error::error::ExpectedError;
    use crate::model::ethereum::EthereumTxLog;
    use crate::schema::ethereum::ethereum_tx_logs;

    pub async fn find_by_queue_index(conn: PgConn, queue_index: String) -> Result<EthereumTxLog, ExpectedError> {
        let tx_log = web::block(move || {
            ethereum_tx_logs::table.filter(ethereum_tx_logs::columns::queue_index.eq(queue_index))
                .first::<EthereumTxLog>(&conn)
        }).await?;
        Ok(tx_log)
    }
}