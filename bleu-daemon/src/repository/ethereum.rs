use std::collections::HashMap;

use diesel::Connection;
use diesel::prelude::*;
use serde_json::{Map, Value};

use crate::error::error::ExpectedError;
use crate::libs::opts::opt_to_result;
use crate::plugin::postgres::Client;
use crate::schema::ethereum::eth_blocks::dsl::*;
use crate::schema::ethereum::eth_txs::dsl::eth_txs;
use crate::types::postgres::PostgresSchema;
use crate::schema::ethereum::{NewEthBlock, EthBlock, NewEthTx, EthTx};

pub fn create_eth_table(conn: Client, schema_map: &HashMap<String, PostgresSchema>) -> Result<(), ExpectedError> {
    for (_, schema) in schema_map.iter() {
        let _ = conn.execute(schema.create_table.as_str());
        for create_index in schema.create_index.iter() {
            let _ = conn.execute(create_index.as_str());
        }
    }
    Ok(())
}

pub fn create_eth_block(conn: Client, block: &Map<String, Value>) -> Result<(), ExpectedError> {
    let new_eth_block = NewEthBlock::new(block);
    let _ = diesel::insert_into(eth_blocks).values(&new_eth_block).get_result::<EthBlock>(&conn)?;
    Ok(())
}

pub fn create_eth_txs(conn: Client, txs: &Vec<Value>) -> Result<(), ExpectedError> {
    for raw_tx in txs.iter() {
        let tx = opt_to_result(raw_tx.as_object())?;
        let new_eth_tx = NewEthTx::new(tx);
        let _ = diesel::insert_into(eth_txs).values(&new_eth_tx).get_result::<EthTx>(&conn)?;
    }
    Ok(())
}
