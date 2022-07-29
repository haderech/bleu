pub mod cosmos_tx {
  use actix_web::web;
  use diesel::prelude::*;

  use crate::config::postgres::Pool;
  use crate::error::error::ExpectedError;
  use crate::model::tx::CosmosTx;
  use crate::repository::pagination::{LoadPaginated, PaginatedRecord};
  use crate::schema::tx::cosmos_tx;
  use crate::schema::tx::cosmos_tx::columns::*;
  use crate::web::Data;

  pub async fn find_tx_by_page_and_count(pool: Data<Pool>, page: i64, count: i64) -> Result<PaginatedRecord<CosmosTx>, ExpectedError> {
    let conn = pool.get()?;
    let paginated_tx = web::block(move || {
      cosmos_tx::table.into_boxed()
        .order(cosmos_tx_id.desc())
        .load_with_pagination(&conn, page, count)
    }).await?;
    Ok(paginated_tx)
  }

  pub async fn find_tx_by_height_and_page_and_count(pool: Data<Pool>, block_height: i64, page: i64, count: i64) -> Result<PaginatedRecord<CosmosTx>, ExpectedError> {
    let conn = pool.get()?;
    let paginated_tx = web::block(move || {
      cosmos_tx::table.into_boxed()
        .filter(height.eq(block_height.to_string()))
        .load_with_pagination(&conn, page, count)
    }).await?;
    Ok(paginated_tx)
  }

  pub async fn find_tx_by_hash(pool: Data<Pool>, hash: String) -> Result<CosmosTx, ExpectedError> {
    let conn = pool.get()?;
    let tx = web::block(move || {
      cosmos_tx::table.filter(txhash.eq(hash))
        .first::<CosmosTx>(&conn)
    }).await?;
    Ok(tx)
  }

  pub async fn find_tx_summary(pool: Data<Pool>) -> Result<Vec<CosmosTx>, ExpectedError> {
    let conn = pool.get()?;
    let tx_summary = web::block(move || {
      cosmos_tx::table.order(cosmos_tx_id.desc())
        .limit(10)
        .load::<CosmosTx>(&conn)
    }).await?;
    Ok(tx_summary)
  }
}
