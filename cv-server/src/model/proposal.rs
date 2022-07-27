use paperclip::actix::Apiv2Schema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Apiv2Schema)]
pub struct CosmosProposal {
  cosmos_proposal_id: i64,
  content: Option<serde_json::Value>,
  deposit_end_time: Option<String>,
  final_tally_result: Option<serde_json::Value>,
  proposal_id: Option<String>,
  status: Option<String>,
  submit_time: Option<String>,
  total_deposit: Option<serde_json::Value>,
  voting_end_time: Option<String>,
  voting_start_time: Option<String>,
}
