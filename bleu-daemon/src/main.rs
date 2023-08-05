use appbase::prelude::*;
use plugin::{
	sync::SyncRpcPlugin, substrate::SubstratePlugin
};

mod error;
mod libs;
mod plugin;
mod types;

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
