table! {
  cosmos_block (cosmos_block_id) {
    cosmos_block_id -> BigInt,
    app_hash -> Nullable<Text>,
    chain_id -> Nullable<Text>,
    consensus_hash -> Nullable<Text>,
    data_hash -> Nullable<Text>,
    evidence_hash -> Nullable<Text>,
    hash -> Nullable<Text>,
    height -> Nullable<Text>,
    last_block_id -> Nullable<Text>,
    last_commit_hash -> Nullable<Text>,
    last_results_hash -> Nullable<Text>,
    next_validators_hash -> Nullable<Text>,
    num_txs -> BigInt,
    proposer_address -> Nullable<Text>,
    time -> Nullable<Text>,
    validators_hash -> Nullable<Text>,
    version -> Nullable<Text>,
  }
}
