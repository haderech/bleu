use crate::{
	error::error::ExpectedError,
	libs::{self, serde::get_string},
	types::{
		channel::MultiSender,
		enumeration::Enumeration,
		sync::{SyncMethod, SyncState, SyncStatus},
	},
};
use serde_json::Value;
use std::fs;

pub fn load_state(sync_type: &str) -> Result<SyncState, ExpectedError> {
	log::debug!("load_sync_state; sync_type={}", sync_type);
	match fs::read_to_string(format!("state/{}.json", sync_type)) {
		Ok(state) => {
			let json_value: Value = serde_json::from_str(&state)?;
			let sync_state = json_value
				.as_object()
				.ok_or(ExpectedError::ParsingError("sync state is not object.".to_string()))?;
			Ok(SyncState::from(sync_state))
		},
		Err(e) => {
			log::error!("{}", e.to_string());
			let new_sync = fs::read_to_string(format!("sync/{}.json", sync_type))?;
			let json_value: Value = serde_json::from_str(new_sync.as_str())?;
			let sync_state = json_value
				.as_object()
				.ok_or(ExpectedError::ParsingError("sync state is not object.".to_string()))?;
			Ok(SyncState::new(sync_state))
		},
	}
}

pub fn error_handler(err: ExpectedError, sync_state: &mut SyncState, senders: &MultiSender) {
	log::debug!("error_handler; err={}", err.to_string());
	match err {
		ExpectedError::BlockHeightError(err) => log::debug!("{}", err.to_string()),
		ExpectedError::FilterError(err) => {
			log::debug!("{}", err.to_string());
			sync_state.next_idx();
		},
		ExpectedError::RequestError(err) => log::error!("{}", err.to_string()),
		_ => {
			log::error!("{}", err.to_string());
			sync_state.handle_error(err.to_string());
			let _ = libs::error::error(senders.get("slack"), err);
		},
	};
}

pub fn save_state(sync_state: &SyncState) -> Result<(), ExpectedError> {
	log::debug!("sync_state; sync_id={}", sync_state.sync_id);
	let json_str = serde_json::to_string_pretty(sync_state)?;
	fs::write(format!("state/{}.json", sync_state.sync_type), json_str)?;
	Ok(())
}

pub fn message_handler(message: Value, sync_state: &mut SyncState) -> Result<(), ExpectedError> {
	log::debug!("message_handler; message={}, sync_id={}", message.to_string(), sync_state.sync_id);
	let parsed_msg = message
		.as_object()
		.ok_or(ExpectedError::ParsingError("message is not object.".to_string()))?;
	let method = get_string(parsed_msg, "method")?;
	let method = SyncMethod::find(method.as_str())
		.ok_or(ExpectedError::InvalidError(format!("{} is not valid method.", method)))?;
	match method {
		SyncMethod::Start => {
			sync_state.status(SyncStatus::Working);
		},
		SyncMethod::Stop => {
			sync_state.status(SyncStatus::Stopped);
		},
	};
	save_state(&sync_state)?;
	Ok(())
}
