pub mod cosmos_block {
  use actix_web::web;
  use diesel::prelude::*;

  use crate::config::postgres::Pool;
  use crate::error::error::ExpectedError;
  use crate::model::block::CosmosBlock;
  use crate::repository::pagination::{LoadPaginated, PaginatedRecord};
  use crate::schema::block::cosmos_block;
  use crate::schema::block::cosmos_block::columns::*;

  pub async fn find_block_by_page_and_count(pool: web::Data<Pool>, page: i64, count: i64) -> Result<PaginatedRecord<CosmosBlock>, ExpectedError> {
    let conn = pool.get()?;
    let paginated_block = web::block(move || {
      cosmos_block::table.into_boxed()
        .order(cosmos_block_id.desc())
        .load_with_pagination(&conn, page, count)
    }).await?;
    Ok(paginated_block)
  }

  pub async fn find_block_by_height(pool: web::Data<Pool>, block_height: i64) -> Result<CosmosBlock, ExpectedError> {
    let conn = pool.get()?;
    let block = web::block(move || {
      cosmos_block::table.filter(height.eq(block_height.to_string()))
        .first::<CosmosBlock>(&conn)
    }).await?;
    Ok(block)
  }
}
