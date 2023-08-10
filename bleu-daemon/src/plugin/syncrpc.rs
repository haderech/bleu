use crate::{
	libs::filedb,
	plugin::jsonrpc::JsonRpcPlugin,
	types::{channel::MultiSender, sync::SyncState},
};
use appbase::prelude::*;
use jsonrpc_core::Params;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[appbase_plugin(JsonRpcPlugin)]
pub struct SyncRpcPlugin {
	senders: Option<MultiSender>,
}

impl Plugin for SyncRpcPlugin {
	fn new() -> Self {
		SyncRpcPlugin { senders: None }
	}

	fn init(&mut self) {
		self.senders = Some(MultiSender::new(vec!["substrate"]));
		self.init_rpc();
	}

	fn startup(&mut self) {}

	fn shutdown(&mut self) {}
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncRpcParams {
	sync_type: String,
}

impl SyncRpcPlugin {
	fn init_rpc(&self) {
		let senders = self.senders.clone().unwrap();
		APP.run_with::<JsonRpcPlugin, _, _>(|jsonrpc| {
			jsonrpc.add_method(String::from("start_sync"), move |params: Params| {
				let SyncRpcParams { sync_type } = match params.parse() {
					Ok(t) => t,
					Err(e) => return Box::new(futures::future::err(e)),
				};
				let sender = senders.get(&sync_type);
				sender.send(Value::String("start_sync".to_string())).unwrap();
				Box::new(futures::future::ok(Value::String(format!(
					"start_sync requested; sync_type: {sync_type}"
				))))
			});
		});

		let senders = self.senders.clone().unwrap();
		APP.run_with::<JsonRpcPlugin, _, _>(|jsonrpc| {
			jsonrpc.add_method(String::from("stop_sync"), move |params: Params| {
				let SyncRpcParams { sync_type } = match params.parse() {
					Ok(t) => t,
					Err(e) => return Box::new(futures::future::err(e)),
				};
				let sender = senders.get(&sync_type);
				sender.send(Value::String("stop_sync".to_string())).unwrap();
				Box::new(futures::future::ok(Value::String(format!(
					"stop_sync requested; sync_type: {sync_type}"
				))))
			});
		});

		APP.run_with::<JsonRpcPlugin, _, _>(|jsonrpc| {
			jsonrpc.add_method(String::from("get_sync"), move |params: Params| {
				let SyncRpcParams { sync_type } = match params.parse() {
					Ok(t) => t,
					Err(e) => return Box::new(futures::future::err(e)),
				};
				let sync_state = filedb::read::<SyncState>("state", &sync_type).unwrap();
				Box::new(futures::future::ok(serde_json::to_value(sync_state).unwrap()))
			});
		});
	}
}
