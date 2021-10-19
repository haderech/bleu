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
        tx_index -> Nullable<BigInt>,
        batch_index -> Nullable<BigInt>,
        batch_number -> Nullable<BigInt>,
        tx_timestamp -> Nullable<BigInt>,
        gas_limit -> Nullable<Text>,
        target -> Nullable<Text>,
        origin -> Nullable<Text>,
        data -> Nullable<Text>,
        queue_origin -> Nullable<Text>,
        value -> Nullable<Text>,
        queue_index -> Nullable<BigInt>,
        decoded -> Nullable<Text>,
        confirmed -> Nullable<Bool>,
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