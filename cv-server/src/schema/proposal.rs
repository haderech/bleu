table! {
  cosmos_proposal (cosmos_proposal_id) {
    cosmos_proposal_id -> BigInt,
    content -> Nullable<Jsonb>,
    deposit_end_time -> Nullable<Text>,
    final_tally_result -> Nullable<Jsonb>,
    proposal_id -> Nullable<Text>,
    default -> Nullable<Text>,
    submit_time -> Nullable<Text>,
    total_deposit -> Nullable<Jsonb>,
    voting_end_time -> Nullable<Text>,
    voting_start_time -> Nullable<Text>,
  }
}
