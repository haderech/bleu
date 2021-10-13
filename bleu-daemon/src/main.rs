use appbase::prelude::*;

use crate::plugin::ethereum::EthereumPlugin;
use crate::plugin::optimism::OptimismPlugin;

mod plugin;
mod types;
mod validation;
mod libs;
mod error;

fn main() {
    env_logger::init();
    APP.register::<OptimismPlugin>();
    APP.init();
    APP.plugin_init::<OptimismPlugin>();
    APP.startup();
    APP.execute();
}
