use actix_web::web;
use diesel::prelude::*;
use diesel::RunQueryDsl;

use crate::error::error::ExpectedError;
use crate::model::ethereum::EthBlock;
use crate::Pool;
use crate::schema::ethereum::eth_blocks::dsl::*;

pub fn find_eth_block_by_id(pool: web::Data<Pool>, block_id: i32) -> Result<EthBlock, ExpectedError> {
    let conn = pool.get()?;
    Ok(eth_blocks.find(block_id).first(&conn)?)
}