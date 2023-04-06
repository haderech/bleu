use crate::{
	config::postgres::Pool,
	error::error::ExpectedError,
	model::{ethereum::*, pagination::RequestPage},
	repository::ethereum::{
		find_block_by_number, find_blocks_by_page_count, find_logs_by_hash, find_tx_by_hash,
		find_txs_by_page_count,
	},
};
use paperclip::actix::{api_v2_operation, web, web::Json};

#[api_v2_operation(tags(EthereumBlock))]
pub async fn get_blocks_by_page_count(
	pool: web::Data<Pool>,
	req_page: web::Query<RequestPage>,
) -> Result<Json<PaginatedEthereumBlock>, ExpectedError> {
	Ok(Json(PaginatedEthereumBlock::new(
		find_blocks_by_page_count(pool, req_page.page, req_page.count).await?,
	)))
}

#[api_v2_operation(tags(EthereumBlock))]
pub async fn get_block_by_number(
	pool: web::Data<Pool>,
	number: web::Path<u64>,
) -> Result<Json<EthereumBlock>, ExpectedError> {
	let number = number.into_inner();
	Ok(Json(find_block_by_number(pool, number).await?))
}

#[api_v2_operation(tags(EthereumTransaction))]
pub async fn get_txs_by_page_count(
	pool: web::Data<Pool>,
	req: web::Query<RequestTxsQuery>,
) -> Result<Json<PaginatedEthereumTransaction>, ExpectedError> {
	let req = req.into_inner();
	Ok(Json(PaginatedEthereumTransaction::new(find_txs_by_page_count(pool, req).await?)))
}

#[api_v2_operation(tags(EthereumTransaction))]
pub async fn get_tx_by_hash(
	pool: web::Data<Pool>,
	hash: web::Path<String>,
) -> Result<Json<EthereumTransaction>, ExpectedError> {
	let hash = hash.into_inner();
	Ok(Json(find_tx_by_hash(pool, hash).await?))
}

#[api_v2_operation(tags(EthereumLog))]
pub async fn get_logs_by_hash(
	pool: web::Data<Pool>,
	hash: web::Path<String>,
) -> Result<Json<Vec<EthereumReceiptLog>>, ExpectedError> {
	let hash = hash.into_inner();
	let logs = find_logs_by_hash(pool, hash)
		.await?
		.iter()
		.map(|log| EthereumReceiptLog::from(log.clone()))
		.collect::<Vec<EthereumReceiptLog>>();
	Ok(Json(logs))
}
