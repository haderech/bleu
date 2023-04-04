use crate::{
	enumeration,
	libs::serde::{get_string, get_string_vec, get_u64},
	types::enumeration::Enumeration,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct SyncState {
	pub sync_id: String,
	pub sync_type: String,
	pub chain_name: String,
	pub chain_id: String,
	pub from_idx: u64,
	pub sync_idx: u64,
	pub endpoints: Vec<String>,
	pub endpoint_idx: u16,
	pub filter: String,
	pub status: SyncStatus,
	pub error_msg: String,
}

impl SyncState {
	pub fn new(params: &Map<String, Value>) -> Self {
		let sync_type = get_string(params, "sync_type").unwrap();
		let chain_name = get_string(params, "chain_name").unwrap();
		let chain_id = get_string(params, "chain_id").unwrap();
		SyncState {
			sync_id: format!("{}::{}::{}", chain_name, chain_id, sync_type),
			sync_type,
			chain_name,
			chain_id,
			from_idx: get_u64(params, "from_idx").unwrap(),
			sync_idx: get_u64(params, "from_idx").unwrap(),
			endpoints: get_string_vec(params, "endpoints").unwrap(),
			endpoint_idx: 0,
			filter: get_string(params, "filter").unwrap(),
			status: SyncStatus::Working,
			error_msg: "".to_string(),
		}
	}

	pub fn from(params: &Map<String, Value>) -> Self {
		SyncState {
			sync_id: get_string(params, "sync_id").unwrap(),
			sync_type: get_string(params, "sync_type").unwrap(),
			chain_name: get_string(params, "chain_name").unwrap(),
			chain_id: get_string(params, "chain_id").unwrap(),
			from_idx: get_u64(params, "from_idx").unwrap(),
			sync_idx: get_u64(params, "sync_idx").unwrap(),
			endpoints: get_string_vec(params, "endpoints").unwrap(),
			endpoint_idx: get_u64(params, "endpoint_idx").unwrap() as u16,
			filter: get_string(params, "filter").unwrap(),
			status: SyncStatus::find(get_string(params, "status").unwrap().as_str()).unwrap(),
			error_msg: get_string(params, "error_msg").unwrap(),
		}
	}

	pub fn is_workable(&self) -> bool {
		vec![SyncStatus::Working].contains(&self.status)
	}

	pub fn handle_error(&mut self, error_msg: String) {
		if usize::from(self.endpoint_idx) + 1 < self.endpoints.len() {
			self.endpoint_idx += 1;
		} else {
			self.status = SyncStatus::Error;
		}
		self.error_msg = error_msg;
	}

	pub fn active_node(&self) -> String {
		let idx = usize::from(self.endpoint_idx);
		self.endpoints[idx].clone()
	}

	pub fn next_idx(&mut self) {
		self.sync_idx += 1;
	}

	pub fn status(&mut self, status: SyncStatus) {
		self.status = status;
	}

	pub fn get_filter(&self) -> String {
		self.filter.clone()
	}
}

enumeration!(SyncStatus; {Working: "Working"}, {Stopped: "Stopped"}, {Error: "Error"});
enumeration!(SyncMethod; {Start: "start"}, {Stop: "stop"});
