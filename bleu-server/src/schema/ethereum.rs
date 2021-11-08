table! {
    ethereum_tx_logs (ethereum_tx_log_id) {
        ethereum_tx_log_id -> BigInt,
        address -> Nullable<Text>,
        topics -> Nullable<Text>,
        data -> Nullable<Text>,
        block_number -> Nullable<Text>,
        tx_hash -> Nullable<Text>,
        tx_index -> Nullable<Text>,
        block_hash -> Nullable<Text>,
        log_index -> Nullable<Text>,
        removed -> Nullable<Bool>,
        queue_index -> Nullable<Text>,
    }
}