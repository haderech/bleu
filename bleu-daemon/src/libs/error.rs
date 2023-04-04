use crate::{
	error::error::ExpectedError,
	plugin::slack::{SlackMsg, SlackMsgLevel},
	types::enumeration::Enumeration,
};
use appbase::prelude::*;

pub fn error_handler(slack_sender: Sender, error: ExpectedError) {
	log::error!("{}", error.to_string());
	if let Err(e) =
		slack_sender.send(SlackMsg::new(SlackMsgLevel::Error.value(), error.to_string()))
	{
		log::error!("failed to send slack message! error={}", e.to_string());
	}
}

pub fn warn_handler(slack_sender: Sender, error: ExpectedError) {
	log::warn!("{}", error.to_string());
	if let Err(e) = slack_sender.send(SlackMsg::new(SlackMsgLevel::Warn.value(), error.to_string()))
	{
		log::error!("failed to send slack message! error={}", e.to_string());
	}
}
