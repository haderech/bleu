table! {
  cosmos_validator (cosmos_validator_id) {
    cosmos_validator_id -> BigInt,
    commission -> Nullable<Jsonb>,
    consensus_pubkey -> Nullable<Jsonb>,
    delegator_shares -> Nullable<Text>,
    description -> Nullable<Jsonb>,
    jailed -> Nullable<Bool>,
    min_self_delegation -> Nullable<Text>,
    operator_address -> Nullable<Text>,
    status -> Nullable<Text>,
    tokens -> Nullable<Text>,
    unbonding_height -> Nullable<Text>,
    unbonding_time -> Nullable<Text>,
    version -> Nullable<BigInt>,
  }
}
