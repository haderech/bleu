table! {
    optimism_batches (optimism_batches_id) {
        optimism_batches_id -> Integer,
        batch_index -> Nullable<Text>,
        timestamp -> Nullable<Text>,
        batch_size -> Nullable<Text>,
        l1_tx_hash -> Nullable<Text>,
        l1_block_number -> Nullable<Text>,
        batch_root -> Nullable<Text>,
        previous_total_elements -> Nullable<Text>,
        extra_data -> Nullable<Text>,
        tx_size -> Nullable<Integer>,
    }
}

table! {
    optimism_txs (optimism_txs_id) {
        optimism_txs_id -> Integer,
        tx_hash -> Nullable<Text>,
        tx_index -> Nullable<Text>,
        timestamp -> Nullable<Text>,
        from_address -> Nullable<Text>,
        to_address -> Nullable<Text>,
        token_transferred -> Nullable<Text>,
        value -> Nullable<Text>,
        tx_fee -> Nullable<Text>,
        ether_price -> Nullable<Text>,
        gas_used_by_tx -> Nullable<Text>,
        nonce -> Nullable<Text>,
        input_data -> Nullable<Text>,
        l1_txn_batch_index -> Nullable<Text>,
        l1_submission_tx_hash -> Nullable<Text>,
        l1_state_batch_index -> Nullable<Text>,
        l1_state_root_submission_tx_hash -> Nullable<Text>,
    }
}
