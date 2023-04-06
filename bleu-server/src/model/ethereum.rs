use crate::{library, model::pagination::PageInfo, repository::pagination::PaginatedRecord};
use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct EthereumBlock {
	ethereum_blocks_id: i64,
	author: Option<String>,
	base_fee_per_gas: Option<String>,
	block_number: Option<String>,
	block_size: Option<String>,
	block_timestamp: Option<String>,
	difficulty: Option<String>,
	extra_data: Option<String>,
	gas_limit: Option<String>,
	gas_used: Option<String>,
	hash: Option<String>,
	logs_bloom: Option<String>,
	miner: Option<String>,
	nonce: Option<String>,
	parent_hash: Option<String>,
	receipt_root: Option<String>,
	sha3_uncles: Option<String>,
	state_root: Option<String>,
	total_difficulty: Option<String>,
	transaction_root: Option<String>,
	uncles: Option<String>,
	txn: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct PaginatedEthereumBlock {
	page_info: PageInfo,
	records: Vec<EthereumBlock>,
}

impl PaginatedEthereumBlock {
	pub fn new(paginated: PaginatedRecord<EthereumBlock>) -> Self {
		Self {
			page_info: PageInfo::new(
				paginated.page,
				paginated.count,
				paginated.total_page,
				paginated.total_count,
			),
			records: paginated.records,
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct EthereumTransaction {
	block_hash: Option<String>,
	block_number: Option<String>,
	chain_id: Option<String>,
	gas: Option<String>,
	gas_price: Option<String>,
	hash: Option<String>,
	max_fee_per_gas: Option<String>,
	nonce: Option<String>,
	public_key: Option<String>,
	tx_from: Option<String>,
	tx_input: Option<String>,
	tx_to: Option<String>,
	tx_type: Option<String>,
	tx_value: Option<String>,
	block_timestamp: Option<String>,
	contract_address: Option<String>,
	cumulative_gas_used: Option<String>,
	effective_gas_price: Option<String>,
	gas_used: Option<String>,
	status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct PaginatedEthereumTransaction {
	page_info: PageInfo,
	records: Vec<EthereumTransaction>,
}

impl PaginatedEthereumTransaction {
	pub fn new(paginated: PaginatedRecord<EthereumTransaction>) -> Self {
		Self {
			page_info: PageInfo::new(
				paginated.page,
				paginated.count,
				paginated.total_page,
				paginated.total_count,
			),
			records: paginated.records,
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct EthereumTxReceipt {
	ethereum_transactions_id: i64,
	access_list: Option<String>,
	block_hash: Option<String>,
	block_number: Option<String>,
	chain_id: Option<String>,
	creates: Option<String>,
	gas: Option<String>,
	gas_price: Option<String>,
	hash: Option<String>,
	max_fee_per_gas: Option<String>,
	nonce: Option<String>,
	public_key: Option<String>,
	r: Option<String>,
	s: Option<String>,
	standard_v: Option<String>,
	transaction_index: Option<String>,
	tx_from: Option<String>,
	tx_input: Option<String>,
	tx_to: Option<String>,
	tx_type: Option<String>,
	tx_value: Option<String>,
	v: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable)]
pub struct EthereumLog {
	ethereum_logs_id: i64,
	address: Option<String>,
	block_hash: Option<String>,
	block_number: Option<String>,
	log_data: Option<String>,
	log_index: Option<String>,
	removed: Option<bool>,
	topics: Option<String>,
	transaction_hash: Option<String>,
	transaction_index: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Apiv2Schema)]
pub struct EthereumReceiptLog {
	address: Option<String>,
	block_hash: Option<String>,
	block_number: Option<String>,
	log_data: Option<String>,
	log_index: Option<String>,
	removed: Option<bool>,
	topics: Option<Vec<String>>,
	transaction_hash: Option<String>,
	transaction_index: Option<String>,
}

impl EthereumReceiptLog {
	pub fn from(log: EthereumLog) -> Self {
		Self {
			address: log.address,
			block_hash: log.block_hash,
			block_number: log.block_number,
			log_data: log.log_data,
			log_index: log.log_index,
			removed: log.removed,
			topics: if let Some(t) = log.topics {
				Some(library::convert::convert_str_to_vec(t))
			} else {
				None
			},
			transaction_hash: log.transaction_hash,
			transaction_index: log.transaction_index,
		}
	}
}

#[derive(Debug, Clone, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestTxsQuery {
	pub number: Option<u64>,
	pub address: Option<String>,
	pub page: i64,
	pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Apiv2Schema)]
pub struct BoardSummary {
	pub latest_block_number: Option<String>,
	pub latest_block_gas_used: Option<String>,
	pub latest_block_gas_limit: Option<String>,
	pub total_transaction_count: i64,
}

impl BoardSummary {
	pub fn new(latest_block: EthereumBlock, total_transaction_count: i64) -> Self {
		Self {
			latest_block_number: latest_block.block_number,
			latest_block_gas_used: latest_block.gas_used,
			latest_block_gas_limit: latest_block.gas_limit,
			total_transaction_count,
		}
	}
}


#[derive(Debug, Clone, Serialize, Deserialize, Apiv2Schema)]
pub struct RequestBlockQuery {
	pub number: Option<u64>,
	pub hash: Option<String>,
}
