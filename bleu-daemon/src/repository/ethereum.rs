use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::error::error::ExpectedError;
use crate::libs::opts::opt_to_result;
use crate::libs::serde::{convert_to_string, find_value};
use crate::plugin::postgres::Connection;
use crate::types::postgres::PostgresSchema;

pub fn create_eth_table(mut conn: Connection, schema_map: &HashMap<String, PostgresSchema>) -> Result<(), ExpectedError> {
    for (_, schema) in schema_map.iter() {
        let _ = conn.execute(schema.create_table.as_str(), &[]);
        for create_index in schema.create_index.iter() {
            let _ = conn.execute(create_index.as_str(), &[]);
        }
    }
    Ok(())
}

pub fn create_eth_block(mut conn: Connection, block: &Map<String, Value>) -> Result<(), ExpectedError> {
    let insert_query = "INSERT INTO eth_blocks \
    (base_fee_per_gas, difficulty, extra_data, gas_limit, gas_used, hash, logs_bloom, miner, mix_hash, nonce, block_number, parent_hash, receipts_root, sha3_uncles, block_size, state_root, block_timestamp, total_difficulty) \
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)";

    let base_fee_per_gas = convert_to_string(find_value(block, "baseFeePerGas"));
    let difficulty = convert_to_string(find_value(block, "difficulty"));
    let extra_data = convert_to_string(find_value(block, "extraData"));
    let gas_limit = convert_to_string(find_value(block, "gasLimit"));
    let gas_used = convert_to_string(find_value(block, "gasUsed"));
    let hash = convert_to_string(find_value(block, "hash"));
    let logs_bloom = convert_to_string(find_value(block, "logsBloom"));
    let miner = convert_to_string(find_value(block, "miner"));
    let mix_hash = convert_to_string(find_value(block, "mixHash"));
    let nonce = convert_to_string(find_value(block, "nonce"));
    let block_number = convert_to_string(find_value(block, "number"));
    let parent_hash = convert_to_string(find_value(block, "parentHash"));
    let receipts_root = convert_to_string(find_value(block, "receiptsRoot"));
    let sha3_uncles = convert_to_string(find_value(block, "sha3Uncles"));
    let block_size = convert_to_string(find_value(block, "size"));
    let state_root = convert_to_string(find_value(block, "stateRoot"));
    let block_timestamp = convert_to_string(find_value(block, "timestamp"));
    let total_difficulty = convert_to_string(find_value(block, "totalDifficulty"));

    let _ = conn.execute(insert_query, &[
        &base_fee_per_gas, &difficulty, &extra_data, &gas_limit, &gas_used, &hash, &logs_bloom, &miner, &mix_hash, &nonce, &block_number, &parent_hash, &receipts_root,
        &sha3_uncles, &block_size, &state_root, &block_timestamp, &total_difficulty
    ]);
    Ok(())
}

pub fn create_eth_txs(mut conn: Connection, txs: &Vec<Value>) -> Result<(), ExpectedError> {
    let insert_query = "INSERT INTO eth_txs \
    (block_hash, block_number, chain_id, from_address, to_address, gas, gas_price, hash, tx_input, max_fee_per_gas, max_priority_fee_per_gas, nonce, tx_index, value) \
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)";
    for raw_tx in txs.iter() {
        let tx = opt_to_result(raw_tx.as_object())?;

        let block_hash = convert_to_string(find_value(tx, "blockHash"));
        let block_number = convert_to_string(find_value(tx, "blockNumber"));
        let chain_id = convert_to_string(find_value(tx, "chainId"));
        let from_address = convert_to_string(find_value(tx, "from"));
        let to_address = convert_to_string(find_value(tx, "to"));
        let gas = convert_to_string(find_value(tx, "gas"));
        let gas_price = convert_to_string(find_value(tx, "gasPrice"));
        let hash = convert_to_string(find_value(tx, "hash"));
        let tx_input = convert_to_string(find_value(tx, "input"));
        let max_fee_per_gas = convert_to_string(find_value(tx, "maxFeePerGas"));
        let max_priority_fee_per_gas = convert_to_string(find_value(tx, "maxPriorityFeePerGas"));
        let nonce = convert_to_string(find_value(tx, "nonce"));
        let tx_index = convert_to_string(find_value(tx, "transactionIndex"));
        let value = convert_to_string(find_value(tx, "value"));

        let _ = conn.execute(insert_query, &[
            &block_hash, &block_number, &chain_id, &from_address, &to_address, &gas, &gas_price, &hash, &tx_input, &max_fee_per_gas, &max_priority_fee_per_gas,
            &nonce, &tx_index, &value
        ]);
    }
    Ok(())
}
