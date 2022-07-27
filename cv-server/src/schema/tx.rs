table! {
  cosmos_tx (cosmos_tx_id) {
    cosmos_tx_id -> BigInt,
    code -> BigInt,
    data -> Nullable<Text>,
    fee -> Nullable<Jsonb>,
    gas_used -> Nullable<Text>,
    gas_wanted -> Nullable<Text>,
    height -> Nullable<Text>,
    memo -> Nullable<Text>,
    messages -> Nullable<Jsonb>,
    raw_log -> Nullable<Text>,
    timestamp -> Nullable<Text>,
    txhash -> Nullable<Text>,
  }
}
