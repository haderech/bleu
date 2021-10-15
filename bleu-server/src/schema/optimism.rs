table! {
    optimism_tx_batches (optimism_tx_batches_id) {
        optimism_tx_batches_id -> BigInt,
        batch_index -> Nullable<BigInt>,
        timestamp -> Nullable<BigInt>,
        batch_size -> Nullable<BigInt>,
        l1_tx_hash -> Nullable<Text>,
        l1_block_number -> Nullable<BigInt>,
        batch_root -> Nullable<Text>,
        previous_total_elements -> Nullable<BigInt>,
        extra_data -> Nullable<Text>,
        submitter -> Nullable<Text>,
    }
}

table! {
    optimism_txs (optimism_txs_id) {
        optimism_txs_id -> BigInt,
        tx_hash -> Nullable<Text>,
        status -> Nullable<Text>,
        tx_index -> Nullable<BigInt>,
        timestamp -> Nullable<BigInt>,
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
        l1_txn_batch_index -> Nullable<BigInt>,
        l1_submission_tx_hash -> Nullable<Text>,
        l1_state_batch_index -> Nullable<BigInt>,
        l1_state_root_submission_tx_hash -> Nullable<Text>,
    }
}

table! {
    optimism_state_batches (optimism_state_batches_id) {
        optimism_state_batches_id -> BigInt,
        batch_index -> Nullable<BigInt>,
        l1_timestamp -> Nullable<BigInt>,
        batch_size -> Nullable<BigInt>,
        l1_tx_hash -> Nullable<Text>,
        l1_block_number -> Nullable<BigInt>,
        batch_root -> Nullable<Text>,
        previous_total_elements -> Nullable<BigInt>,
        extra_data -> Nullable<Text>,
        submitter -> Nullable<Text>,
    }
}

table! {
    optimism_state_roots (optimism_state_roots_id) {
        optimism_state_roots_id -> BigInt,
        index -> Nullable<BigInt>,
        batch_index -> Nullable<BigInt>,
        value -> Nullable<Text>,
        confirmed -> Nullable<Bool>,
    }
}

table! {
    optimism_l1_to_l2_txs (optimism_l1_to_l2_txs_id) {
        optimism_l1_to_l2_txs_id -> BigInt,
        l1_block_number -> Nullable<BigInt>,
        l1_tx_hash -> Nullable<Text>,
        l2_tx_hash -> Nullable<Text>,
    }
}
