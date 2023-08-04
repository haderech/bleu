use appbase::prelude::*;
use plugins::{
	substrate::SubstratePlugin,
	sync::SyncRpcPlugin,
};

mod error;
mod libs;
mod plugins;
mod types;
mod utils;

fn main() {
	env_logger::init();
	APP.register::<SubstratePlugin>();
	APP.register::<SyncRpcPlugin>();
	APP.init();
	APP.plugin_init::<SubstratePlugin>();
	APP.plugin_init::<SyncRpcPlugin>();
	APP.startup();
	APP.execute();
}
