use crate::error::error::ExpectedError;
use appbase::prelude::*;
use serde_json::{Map, Value};

#[allow(dead_code)]
pub fn error(sender: Sender, e: ExpectedError) {
	let mut message = Map::new();
	message.insert("level".to_string(), Value::from("error"));
	message.insert("message".to_string(), Value::from(e.to_string()));
	if let Err(e) = sender.send(Value::from(message)) {
		log::error!("this error will be ignored; {}", e.to_string());
	}
}

#[allow(dead_code)]
pub fn warn(sender: Sender, e: ExpectedError) {
	let mut message = Map::new();
	message.insert("level".to_string(), Value::from("warn"));
	message.insert("message".to_string(), Value::from(e.to_string()));
	if let Err(e) = sender.send(Value::from(message)) {
		log::error!("this error will be ignored; {}", e.to_string());
	}
}
