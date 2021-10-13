table! {
    optimism_batches (optimism_batches_id) {
        optimism_batches_id -> Integer,
        batch_index -> Nullable<Integer>,
        timestamp -> Nullable<Integer>,
        batch_size -> Nullable<Integer>,
        l1_tx_hash -> Nullable<Text>,
        l1_block_number -> Nullable<Integer>,
        batch_root -> Nullable<Text>,
        previous_total_elements -> Nullable<Integer>,
        extra_data -> Nullable<Text>,
        submitter -> Nullable<Text>,
    }
}

table! {
    optimism_txs (optimism_txs_id) {
        optimism_txs_id -> Integer,
        tx_hash -> Nullable<Text>,
        status -> Nullable<Text>,
        tx_index -> Nullable<Integer>,
        timestamp -> Nullable<Integer>,
        from_address -> Nullable<Text>,
        to_address -> Nullable<Text>,
        token_transferred -> Nullable<Text>,
        value -> Nullable<Text>,
        tx_fee -> Nullable<Text>,
        ether_price -> Nullable<Text>,
        gas_used_by_tx -> Nullable<Text>,
        nonce -> Nullable<Text>,
        input_data -> Nullable<Text>,
        confirmed -> Nullable<Bool>,
        l1_txn_batch_index -> Nullable<Text>,
        l1_submission_tx_hash -> Nullable<Text>,
        l1_state_batch_index -> Nullable<Text>,
        l1_state_root_submission_tx_hash -> Nullable<Text>,
    }
}

table! {
    optimism_l1_to_l2_txs (optimism_l1_to_l2_txs_id) {
        optimism_l1_to_l2_txs_id -> Integer,
        l1_block_number -> Nullable<Text>,
        l1_tx_hash -> Nullable<Text>,
        l2_tx_hash -> Nullable<Text>,
    }
}
