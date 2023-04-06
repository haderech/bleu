table! {
	ethereum_blocks (ethereum_blocks_id) {
		ethereum_blocks_id -> BigInt,
		author -> Nullable<Text>,
		base_fee_per_gas -> Nullable<Text>,
		block_number -> Nullable<Text>,
		block_size -> Nullable<Text>,
		block_timestamp -> Nullable<Text>,
		difficulty -> Nullable<Text>,
		extra_data -> Nullable<Text>,
		gas_limit -> Nullable<Text>,
		gas_used -> Nullable<Text>,
		hash -> Nullable<Text>,
		logs_bloom -> Nullable<Text>,
		miner -> Nullable<Text>,
		nonce -> Nullable<Text>,
		parent_hash -> Nullable<Text>,
		receipts_root -> Nullable<Text>,
		sha3_uncles -> Nullable<Text>,
		state_root -> Nullable<Text>,
		total_difficulty -> Nullable<Text>,
		transactions_root -> Nullable<Text>,
		uncles -> Nullable<Text>,
		txn -> BigInt,
	}
}

table! {
	ethereum_transactions (ethereum_transactions_id) {
		ethereum_transactions_id -> BigInt,
		access_list -> Nullable<Text>,
		block_hash -> Nullable<Text>,
		block_number -> Nullable<Text>,
		chain_id -> Nullable<Text>,
		creates -> Nullable<Text>,
		gas -> Nullable<Text>,
		gas_price -> Nullable<Text>,
		hash -> Nullable<Text>,
		max_fee_per_gas -> Nullable<Text>,
		nonce -> Nullable<Text>,
		public_key -> Nullable<Text>,
		r -> Nullable<Text>,
		s -> Nullable<Text>,
		standard_v -> Nullable<Text>,
		transaction_index -> Nullable<Text>,
		tx_from -> Nullable<Text>,
		tx_input -> Nullable<Text>,
		tx_to -> Nullable<Text>,
		tx_type -> Nullable<Text>,
		tx_value -> Nullable<Text>,
		v -> Nullable<Text>,
	}
}

table! {
	ethereum_tx_receipts (ethereum_tx_receipts_id) {
		ethereum_tx_receipts_id -> BigInt,
		block_hash -> Nullable<Text>,
		block_number -> Nullable<Text>,
		contract_address -> Nullable<Text>,
		cumulative_gas_used -> Nullable<Text>,
		effective_gas_price -> Nullable<Text>,
		gas_used -> Nullable<Text>,
		logs_bloom -> Nullable<Text>,
		status -> Nullable<Text>,
		transaction_hash -> Nullable<Text>,
		transaction_index -> Nullable<Text>,
		tx_from -> Nullable<Text>,
		tx_to -> Nullable<Text>,
		tx_type -> Nullable<Text>,
	}
}

table! {
	ethereum_logs (ethereum_logs_id) {
		ethereum_logs_id -> BigInt,
		address -> Nullable<Text>,
		block_hash -> Nullable<Text>,
		block_number -> Nullable<Text>,
		log_data -> Nullable<Text>,
		log_index -> Nullable<Text>,
		removed -> Nullable<Bool>,
		topics -> Nullable<Text>,
		transaction_hash -> Nullable<Text>,
		transaction_index -> Nullable<Text>,
	}
}

joinable_inner!(
	left_table_ty = ethereum_transactions::table,
	right_table_ty = ethereum_blocks::table,
	right_table_expr = ethereum_blocks::table,
	foreign_key = ethereum_transactions::dsl::block_hash,
	primary_key_ty = ethereum_blocks::dsl::hash,
	primary_key_expr = ethereum_blocks::dsl::hash,
);

joinable_inner!(
	left_table_ty = ethereum_transactions::table,
	right_table_ty = ethereum_tx_receipts::table,
	right_table_expr = ethereum_tx_receipts::table,
	foreign_key = ethereum_transactions::dsl::hash,
	primary_key_ty = ethereum_tx_receipts::dsl::transaction_hash,
	primary_key_expr = ethereum_tx_receipts::dsl::transaction_hash,
);

allow_tables_to_appear_in_same_query!(ethereum_transactions, ethereum_blocks, ethereum_tx_receipts);