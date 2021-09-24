use actix_web::{HttpResponse, web};

use crate::error::error::ExpectedError;
use crate::Pool;
use crate::repository::ethereum::find_eth_block_by_id;

pub async fn get_eth_block_by_id(pool: web::Data<Pool>, eth_blocks_id: web::Path<(i32)>) -> Result<HttpResponse, ExpectedError> {
    let eth_blocks_id = eth_blocks_id.into_inner();
    let eth_block = find_eth_block_by_id(pool, eth_blocks_id)?;

    Ok(HttpResponse::Ok().json(eth_block))
}