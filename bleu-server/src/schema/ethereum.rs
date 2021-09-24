table! {
    eth_blocks (eth_blocks_id) {
        eth_blocks_id -> Integer,
        base_fee_per_gas -> Text,
        difficulty -> Text,
        extra_data -> Text,
        gas_limit -> Text,
        gas_used -> Text,
        hash -> Text,
        logs_bloom -> Text,
        miner -> Text,
        mix_hash -> Text,
        nonce -> Text,
        block_number -> Text,
        parent_hash -> Text,
        receipts_root -> Text,
        sha3_uncles -> Text,
        block_size -> Text,
        state_root -> Text,
        block_timestamp -> Text,
        total_difficulty -> Text,
        is_forked -> Bool,
    }
}

table! {
    eth_txs (eth_txs_id) {
        eth_txs_id -> Integer,
        block_hash -> Text,
        block_number -> Text,
        chain_id -> Text,
        from_address -> Text,
        to_address -> Text,
        gas -> Text,
        gas_price -> Text,
        hash -> Text,
        tx_input -> Text,
        max_fee_per_gas -> Text,
        max_priority_fee_per_gas -> Text,
        nonce -> Text,
        tx_index -> Text,
        value -> Text,
        is_forked -> Bool,
    }
}