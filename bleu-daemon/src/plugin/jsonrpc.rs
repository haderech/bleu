use appbase::prelude::*;
use clap::Arg;
use jsonrpc_core::{IoHandler, RpcMethodSimple, RpcMethodSync};
use jsonrpc_http_server::{CloseHandle, ServerBuilder};
use std::{
	net::{IpAddr, Ipv4Addr, SocketAddr},
	str::FromStr,
};

#[appbase_plugin]
pub struct JsonRpcPlugin {
	io: Option<IoHandler>,
	server: Option<CloseHandle>,
}

/*
 * `add_sync_method` and `add_method` SHOULD be called during plugin initialization.
 * After JsonRpcPlugin starts, IoHandler moves into closure, so not available to access from
 * plugin.
 */
impl JsonRpcPlugin {
	#[allow(dead_code)]
	pub fn add_sync_method<F>(&mut self, name: String, func: F)
	where
		F: RpcMethodSync,
	{
		match self.io.as_mut() {
			Some(io) => io.add_sync_method(name.as_str(), func),
			None => log::error!("add method not available"),
		}
	}

	#[allow(dead_code)]
	pub fn add_method<F>(&mut self, name: String, func: F)
	where
		F: RpcMethodSimple,
	{
		match self.io.as_mut() {
			Some(io) => io.add_method(name.as_str(), func),
			None => log::error!("add method not available"),
		}
	}
}

impl Plugin for JsonRpcPlugin {
	fn new() -> Self {
		APP.options
			.arg(Arg::new("jsonrpc::host").long("jsonrpc-host").takes_value(true));
		APP.options
			.arg(Arg::new("jsonrpc::port").long("jsonrpc-port").takes_value(true));
		JsonRpcPlugin { io: None, server: None }
	}

	fn init(&mut self) {
		self.io = Some(IoHandler::new());
	}

	fn startup(&mut self) {
		let host = APP.options.value_of("jsonrpc::host").unwrap_or(String::from("0.0.0.0"));
		let port = APP.options.value_of_t::<u16>("jsonrpc::port").unwrap_or(9999);
		let io = self.io.take().unwrap();
		let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::from_str(&host).unwrap()), port);
		if let Ok(server) = ServerBuilder::new(io).start_http(&socket) {
			self.server = Some(server.close_handle());
			APP.spawn_blocking(|| {
				server.wait();
			});
		}
	}

	fn shutdown(&mut self) {
		if let Some(server) = self.server.take() {
			server.close();
		}
	}
}
