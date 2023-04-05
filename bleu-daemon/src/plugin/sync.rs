use crate::{
	enumeration,
	error::error::ExpectedError,
	libs::serde::get_string,
	message,
	plugin::jsonrpc::JsonRpcPlugin,
	types::{channel::MultiSender, enumeration::Enumeration, sync::SyncMethod},
};
use appbase::prelude::*;
use jsonrpc_core::{serde_json::Map, Params};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs;

#[appbase_plugin(JsonRpcPlugin)]
pub struct SyncRpcPlugin {
	senders: Option<MultiSender>,
}

enumeration!(SyncType; {EthereumBlock: "ethereum_block"});
message!(SyncManageMsg; {method: String});

impl Plugin for SyncRpcPlugin {
	fn new() -> Self {
		SyncRpcPlugin { senders: None }
	}

	fn init(&mut self) {
		self.senders = Some(MultiSender::new(vec!["ethereum_block"]));
		self.add_methods();
	}

	fn startup(&mut self) {}

	fn shutdown(&mut self) {}
}

impl SyncRpcPlugin {
	fn add_methods(&self) {
		let senders = self.senders.clone().unwrap();
		APP.run_with::<JsonRpcPlugin, _, _>(|jsonrpc| {
			jsonrpc.add_method(String::from("start_sync"), move |params: Params| {
				let response = match Self::manage_sync(SyncMethod::Start, params, &senders) {
					Ok(response) => response,
					Err(err) => json!({"error": err.to_string()}),
				};
				Box::new(futures::future::ok(response))
			});
		});

		let senders = self.senders.clone().unwrap();
		APP.run_with::<JsonRpcPlugin, _, _>(|jsonrpc| {
			jsonrpc.add_method(String::from("stop_sync"), move |params: Params| {
				let response = match Self::manage_sync(SyncMethod::Stop, params, &senders) {
					Ok(response) => response,
					Err(err) => json!({"error": err.to_string()}),
				};
				Box::new(futures::future::ok(response))
			});
		});

		APP.run_with::<JsonRpcPlugin, _, _>(|jsonrpc| {
			jsonrpc.add_method(String::from("get_sync"), move |params: Params| {
				let response = match Self::get_sync(params) {
					Ok(response) => response,
					Err(err) => json!({"error": err.to_string()}),
				};
				Box::new(futures::future::ok(response))
			});
		});
	}

	fn manage_sync(
		method: SyncMethod,
		params: Params,
		senders: &MultiSender,
	) -> Result<Value, ExpectedError> {
		let params: Map<String, Value> = params.parse().unwrap();
		let sync_type = get_string(&params, "sync_type")?;
		let sender = senders.get(&sync_type);
		let _ = sender.send(SyncManageMsg::new(method.value()))?;
		Ok(Value::String(format!("requested. sync_type={}, method={}", sync_type, method.value())))
	}

	fn get_sync(params: Params) -> Result<Value, ExpectedError> {
		let params: Map<String, Value> = params.parse().unwrap();
		let sync_type = get_string(&params, "sync_type")?;
		let sync_state = fs::read_to_string(format!("state/{}.json", sync_type))?;
		let state_json = serde_json::from_str(sync_state.as_str())?;
		Ok(Value::Object(state_json))
	}
}
