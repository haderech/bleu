use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum SyncStatus {
	Working,
	Stopped,
	Error,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct SyncState {
	pub sync_type: String,
	pub chain_id: String,
	pub from_idx: u64,
	pub sync_idx: u64,
	pub endpoint: String,
	pub status: SyncStatus,
	pub message: String,
}
