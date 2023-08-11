use super::filedb;
use crate::{
	error::error::ExpectedError,
	types::sync::{SyncState, SyncStatus},
};

pub fn save_state(state: &SyncState) -> Result<(), ExpectedError> {
	filedb::write::<SyncState>("state", &state.sync_type, state)
}

pub fn load_state(sync_type: &str) -> Result<SyncState, ExpectedError> {
	filedb::read::<SyncState>("state", sync_type)
}

pub fn init_state(state: &SyncState) -> Result<(), ExpectedError> {
	filedb::write::<SyncState>("state", &state.sync_type, state)
}

pub fn error_state(e: ExpectedError, state: &mut SyncState) -> Result<(), ExpectedError> {
	state.status = SyncStatus::Error;
	state.message = format!("{}", e);
	filedb::write::<SyncState>("state", &state.sync_type, state)
}

pub fn control_state(method: &str, state: &mut SyncState) -> Result<(), ExpectedError> {
	match method {
		"start_sync" => {
			state.status = SyncStatus::Working;
		},
		"stop_sync" => {
			state.status = SyncStatus::Stopped;
		},
		_ =>
			return Err(ExpectedError::TypeError(
				"unsupported method; [\"start_sync\", \"stop_sync\"]".to_string(),
			)),
	}
	filedb::write::<SyncState>("state", &state.sync_type, state)
}
