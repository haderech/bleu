use crate::{
	error::error::ExpectedError,
	plugin::slack::{SlackMsg, SlackMsgLevel},
	types::enumeration::Enumeration,
};
use appbase::prelude::*;

#[allow(dead_code)]
pub fn error(sender: Sender, error: ExpectedError) {
	log::error!("{}", error.to_string());
	if let Err(e) = sender.send(SlackMsg::new(SlackMsgLevel::Error.value(), error.to_string())) {
		log::error!("failed to send slack message. error={}", e.to_string());
	}
}

#[allow(dead_code)]
pub fn error_message(sender: Sender, message: String) {
	log::error!("{}", message);
	if let Err(e) = sender.send(SlackMsg::new(SlackMsgLevel::Error.value(), message)) {
		log::error!("failed to send slack message. error={}", e.to_string());
	}
}

#[allow(dead_code)]
pub fn warn(sender: Sender, error: ExpectedError) {
	log::warn!("{}", error.to_string());
	if let Err(e) = sender.send(SlackMsg::new(SlackMsgLevel::Warn.value(), error.to_string())) {
		log::error!("failed to send slack message. error={}", e.to_string());
	}
}
