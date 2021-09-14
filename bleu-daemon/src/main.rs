use appbase::prelude::*;

use crate::plugin::ethereum::EthereumPlugin;

mod plugin;
mod types;
mod validation;
mod libs;
mod error;
mod repository;

fn main() {
    env_logger::init();
    APP.register::<EthereumPlugin>();
    APP.init();
    APP.plugin_init::<EthereumPlugin>();
    APP.startup();
    APP.execute();
}
