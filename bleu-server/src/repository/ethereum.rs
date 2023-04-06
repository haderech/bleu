use crate::{
	config::postgres::Pool,
	error::error::ExpectedError,
	model::ethereum::{EthereumBlock, EthereumLog, EthereumTransaction, RequestTxsQuery},
	repository::pagination::{LoadPaginated, PaginatedRecord},
	schema::ethereum::{
		ethereum_blocks, ethereum_logs, ethereum_transactions, ethereum_tx_receipts,
	},
};
use actix_web::web;
use diesel::prelude::*;

pub async fn find_blocks_by_page_count(
	pool: web::Data<Pool>,
	page: i64,
	count: i64,
) -> Result<PaginatedRecord<EthereumBlock>, ExpectedError> {
	let conn = pool.get()?;
	let paginated_blocks = web::block(move || {
		ethereum_blocks::table
			.order(ethereum_blocks::columns::ethereum_blocks_id.desc())
			.load_with_pagination(&conn, page, count)
	})
	.await?;
	Ok(paginated_blocks)
}

pub async fn find_block_by_number(
	pool: web::Data<Pool>,
	number: u64,
) -> Result<EthereumBlock, ExpectedError> {
	let conn = pool.get()?;
	let block = web::block(move || {
		ethereum_blocks::table
			.filter(ethereum_blocks::columns::block_number.eq(number.to_string()))
			.get_result::<EthereumBlock>(&conn)
	})
	.await?;
	Ok(block)
}

pub async fn find_txs_by_page_count(
	pool: web::Data<Pool>,
	req: RequestTxsQuery,
) -> Result<PaginatedRecord<EthereumTransaction>, ExpectedError> {
	let conn = pool.get()?;
	let paginated_txs = web::block(move || {
		let query =
			ethereum_transactions::table
				.left_join(ethereum_blocks::table.on(
					ethereum_transactions::columns::block_hash.eq(ethereum_blocks::columns::hash),
				))
				.left_join(
					ethereum_tx_receipts::table.on(ethereum_transactions::columns::hash
						.eq(ethereum_tx_receipts::columns::transaction_hash)),
				)
				.order(ethereum_transactions::columns::ethereum_transactions_id.desc())
				.select((
					ethereum_transactions::columns::block_hash.nullable(),
					ethereum_transactions::columns::block_number.nullable(),
					ethereum_transactions::columns::chain_id.nullable(),
					ethereum_transactions::columns::gas.nullable(),
					ethereum_transactions::columns::gas_price.nullable(),
					ethereum_transactions::columns::hash.nullable(),
					ethereum_transactions::columns::max_fee_per_gas.nullable(),
					ethereum_transactions::columns::nonce.nullable(),
					ethereum_transactions::columns::public_key.nullable(),
					ethereum_transactions::columns::tx_from.nullable(),
					ethereum_transactions::columns::tx_input.nullable(),
					ethereum_transactions::columns::tx_to.nullable(),
					ethereum_transactions::columns::tx_type.nullable(),
					ethereum_transactions::columns::tx_value.nullable(),
					ethereum_blocks::columns::block_timestamp.nullable(),
					ethereum_tx_receipts::columns::contract_address.nullable(),
					ethereum_tx_receipts::columns::cumulative_gas_used.nullable(),
					ethereum_tx_receipts::columns::effective_gas_price.nullable(),
					ethereum_tx_receipts::columns::gas_used.nullable(),
					ethereum_tx_receipts::columns::status.nullable(),
				));
		if req.number.is_some() {
			let number = req.number.unwrap().to_string();
			query
				.filter(ethereum_transactions::columns::block_number.eq(number))
				.load_with_pagination(&conn, req.page, req.count)
		} else if req.address.is_some() {
			let address = req.address.clone().unwrap();
			query
				.filter(
					ethereum_transactions::columns::tx_from
						.eq(&address)
						.or(ethereum_transactions::columns::tx_to.eq(&address)),
				)
				.load_with_pagination(&conn, req.page, req.count)
		} else {
			query.load_with_pagination(&conn, req.page, req.count)
		}
	})
	.await?;
	Ok(paginated_txs)
}

pub async fn find_tx_by_hash(
	pool: web::Data<Pool>,
	hash: String,
) -> Result<EthereumTransaction, ExpectedError> {
	let conn = pool.get()?;
	let transaction =
		web::block(move || {
			ethereum_transactions::table
				.left_join(ethereum_blocks::table.on(
					ethereum_transactions::columns::block_hash.eq(ethereum_blocks::columns::hash),
				))
				.left_join(
					ethereum_tx_receipts::table.on(ethereum_transactions::columns::hash
						.eq(ethereum_tx_receipts::columns::transaction_hash)),
				)
				.order(ethereum_transactions::columns::ethereum_transactions_id.desc())
				.select((
					ethereum_transactions::columns::block_hash.nullable(),
					ethereum_transactions::columns::block_number.nullable(),
					ethereum_transactions::columns::chain_id.nullable(),
					ethereum_transactions::columns::gas.nullable(),
					ethereum_transactions::columns::gas_price.nullable(),
					ethereum_transactions::columns::hash.nullable(),
					ethereum_transactions::columns::max_fee_per_gas.nullable(),
					ethereum_transactions::columns::nonce.nullable(),
					ethereum_transactions::columns::public_key.nullable(),
					ethereum_transactions::columns::tx_from.nullable(),
					ethereum_transactions::columns::tx_input.nullable(),
					ethereum_transactions::columns::tx_to.nullable(),
					ethereum_transactions::columns::tx_type.nullable(),
					ethereum_transactions::columns::tx_value.nullable(),
					ethereum_blocks::columns::block_timestamp.nullable(),
					ethereum_tx_receipts::columns::contract_address.nullable(),
					ethereum_tx_receipts::columns::cumulative_gas_used.nullable(),
					ethereum_tx_receipts::columns::effective_gas_price.nullable(),
					ethereum_tx_receipts::columns::gas_used.nullable(),
					ethereum_tx_receipts::columns::status.nullable(),
				))
				.filter(ethereum_transactions::columns::hash.eq(hash))
				.get_result::<EthereumTransaction>(&conn)
		})
		.await?;
	Ok(transaction)
}

pub async fn find_logs_by_hash(
	pool: web::Data<Pool>,
	hash: String,
) -> Result<Vec<EthereumLog>, ExpectedError> {
	let conn = pool.get()?;
	let logs = web::block(move || {
		ethereum_logs::table
			.filter(ethereum_logs::columns::transaction_hash.eq(hash))
			.get_results::<EthereumLog>(&conn)
	})
	.await?;
	Ok(logs)
}
