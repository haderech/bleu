use crate::schema::ethereum::ethereum_tx_logs;

table! {
    optimism_tx_batches (optimism_tx_batches_id) {
        optimism_tx_batches_id -> BigInt,
        batch_index -> Nullable<Text>,
        batch_timestamp -> Nullable<Text>,
        batch_size -> Nullable<Text>,
        l1_tx_hash -> Nullable<Text>,
        l1_block_number -> Nullable<Text>,
        batch_root -> Nullable<Text>,
        previous_total_elements -> Nullable<Text>,
        extra_data -> Nullable<Text>,
        submitter -> Nullable<Text>,
    }
}

table! {
    optimism_txs (optimism_txs_id) {
        optimism_txs_id -> BigInt,
        index -> Nullable<Text>,
        batch_index -> Nullable<Text>,
        batch_number -> Nullable<Text>,
        tx_timestamp -> Nullable<Text>,
        gas_limit -> Nullable<Text>,
        target -> Nullable<Text>,
        origin -> Nullable<Text>,
        data -> Nullable<Text>,
        queue_origin -> Nullable<Text>,
        value -> Nullable<Text>,
        queue_index -> Nullable<Text>,
        decoded -> Nullable<Text>,
        confirmed -> Nullable<Bool>,
    }
}

table! {
    optimism_state_batches (optimism_state_batches_id) {
        optimism_state_batches_id -> BigInt,
        batch_index -> Nullable<Text>,
        batch_timestamp -> Nullable<Text>,
        batch_size -> Nullable<Text>,
        l1_tx_hash -> Nullable<Text>,
        l1_block_number -> Nullable<Text>,
        batch_root -> Nullable<Text>,
        previous_total_elements -> Nullable<Text>,
        extra_data -> Nullable<Text>,
        submitter -> Nullable<Text>,
    }
}

table! {
    optimism_state_roots (optimism_state_roots_id) {
        optimism_state_roots_id -> BigInt,
        index -> Nullable<Text>,
        batch_index -> Nullable<Text>,
        value -> Nullable<Text>,
        confirmed -> Nullable<Bool>,
    }
}

table! {
    optimism_blocks (optimism_blocks_id) {
        optimism_blocks_id -> BigInt,
        difficulty -> Nullable<Text>,
        extra_data -> Nullable<Text>,
        gas_limit -> Nullable<Text>,
        gas_used -> Nullable<Text>,
        hash -> Nullable<Text>,
        logs_bloom -> Nullable<Text>,
        miner -> Nullable<Text>,
        mix_hash -> Nullable<Text>,
        nonce -> Nullable<Text>,
        block_number -> Nullable<Text>,
        parent_hash -> Nullable<Text>,
        receipts_root -> Nullable<Text>,
        sha3_uncles -> Nullable<Text>,
        block_size -> Nullable<Text>,
        state_root -> Nullable<Text>,
        block_timestamp -> Nullable<Text>,
        total_difficulty -> Nullable<Text>,
    }
}

table! {
    optimism_block_txs (optimism_block_txs_id) {
        optimism_block_txs_id -> BigInt,
        block_hash -> Nullable<Text>,
        block_number -> Nullable<Text>,
        from_address -> Nullable<Text>,
        gas -> Nullable<Text>,
        gas_price -> Nullable<Text>,
        hash -> Nullable<Text>,
        index -> Nullable<Text>,
        tx_input -> Nullable<Text>,
        l1_block_number -> Nullable<Text>,
        l1_timestamp -> Nullable<Text>,
        l1_tx_origin -> Nullable<Text>,
        nonce -> Nullable<Text>,
        queue_index -> Nullable<Text>,
        queue_origin -> Nullable<Text>,
        raw_tx -> Nullable<Text>,
        to_address -> Nullable<Text>,
        tx_index -> Nullable<Text>,
        tx_type -> Nullable<Text>,
        value -> Nullable<Text>,
    }
}

table! {
    optimism_tx_receipt_logs (optimism_tx_receipt_logs_id) {
        optimism_tx_receipt_logs_id -> BigInt,
        address -> Nullable<Text>,
        topics -> Nullable<Text>,
        data -> Nullable<Text>,
        block_number -> Nullable<Text>,
        tx_hash -> Nullable<Text>,
        tx_index -> Nullable<Text>,
        block_hash -> Nullable<Text>,
        log_index -> Nullable<Text>,
        removed -> Nullable<Bool>,
    }
}

joinable_inner!(
    left_table_ty = optimism_txs::table,
    right_table_ty = optimism_block_txs::table,
    right_table_expr = optimism_block_txs::table,
    foreign_key = optimism_block_txs::dsl::index,
    primary_key_ty = optimism_txs::dsl::index,
    primary_key_expr = optimism_txs::dsl::index,
);

joinable_inner!(
    left_table_ty = optimism_tx_batches::table,
    right_table_ty = optimism_txs::table,
    right_table_expr = optimism_txs::table,
    foreign_key = optimism_txs::dsl::batch_index,
    primary_key_ty = optimism_tx_batches::dsl::batch_index,
    primary_key_expr = optimism_tx_batches::dsl::batch_index,
);

joinable_inner!(
    left_table_ty = optimism_state_roots::table,
    right_table_ty = optimism_block_txs::table,
    right_table_expr = optimism_block_txs::table,
    foreign_key = optimism_block_txs::dsl::index,
    primary_key_ty = optimism_state_roots::dsl::index,
    primary_key_expr = optimism_state_roots::dsl::index,
);

joinable_inner!(
    left_table_ty = optimism_block_txs::table,
    right_table_ty = ethereum_tx_logs::table,
    right_table_expr = ethereum_tx_logs::table,
    foreign_key = ethereum_tx_logs::dsl::queue_index,
    primary_key_ty = optimism_block_txs::dsl::queue_index,
    primary_key_expr = optimism_block_txs::dsl::queue_index,
);

allow_tables_to_appear_in_same_query!(optimism_block_txs, optimism_txs);
allow_tables_to_appear_in_same_query!(optimism_block_txs, optimism_state_roots);
allow_tables_to_appear_in_same_query!(optimism_block_txs, ethereum_tx_logs);