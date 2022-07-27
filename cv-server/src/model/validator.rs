use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct CosmosValidator {
  cosmos_validator_id: i64,
  commission: Option<serde_json::Value>,
  consensus_pubkey: Option<serde_json::Value>,
  delegator_shares: Option<String>,
  description: Option<serde_json::Value>,
  jailed: Option<bool>,
  min_self_delegation: Option<String>,
  operator_address: Option<String>,
  status: Option<String>,
  tokens: Option<String>,
  unbonding_height: Option<String>,
  unbonding_time: Option<String>,
  version: Option<i64>,
}
