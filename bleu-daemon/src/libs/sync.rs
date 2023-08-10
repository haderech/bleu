use super::filedb;
use crate::{
	error::error::ExpectedError,
	types::sync::{SyncState, SyncStatus},
};

pub fn load_state(sync_type: &str) -> Result<SyncState, ExpectedError> {
	filedb::read::<SyncState>("state", sync_type)
}

pub fn init_state(sync_state: &SyncState) -> Result<(), ExpectedError> {
	filedb::write::<SyncState>("state", &sync_state.sync_type, sync_state)
}

pub fn error_state(e: ExpectedError, sync_state: &mut SyncState) -> Result<(), ExpectedError> {
	sync_state.status = SyncStatus::Error;
	sync_state.message = format!("{}", e);
	filedb::write::<SyncState>("state", &sync_state.sync_type, sync_state)
}

pub fn control_state(method: &str, sync_state: &mut SyncState) -> Result<(), ExpectedError> {
	match method {
		"start_sync" => {
			sync_state.status = SyncStatus::Working;
		},
		"stop_sync" => {
			sync_state.status = SyncStatus::Stopped;
		},
		_ =>
			return Err(ExpectedError::TypeError(
				"unsupported method; [\"start_sync\", \"stop_sync\"]".to_string(),
			)),
	}
	filedb::write::<SyncState>("state", &sync_state.sync_type, sync_state)
}
