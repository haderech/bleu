use appbase::prelude::*;

use crate::plugin::l1_tx_log::L1TxLogPlugin;
use crate::plugin::l2_block_tx::L2BlockTxPlugin;
use crate::plugin::l2_enqueue::L2EnqueuePlugin;
use crate::plugin::l2_state_batch::L2StateBatchPlugin;
use crate::plugin::l2_tx_batch::L2TxBatchPlugin;
use crate::plugin::l2_tx_receipt::L2TxReceiptPlugin;
use crate::plugin::task::TaskPlugin;

mod plugin;
mod types;
mod validation;
mod libs;
mod error;

fn main() {
    env_logger::init();
    APP.register::<L2BlockTxPlugin>();
    APP.register::<L2TxBatchPlugin>();
    APP.register::<L2StateBatchPlugin>();
    APP.register::<L2TxReceiptPlugin>();
    APP.register::<L2EnqueuePlugin>();
    APP.register::<L1TxLogPlugin>();
    APP.register::<TaskPlugin>();
    APP.init();
    APP.plugin_init::<L2BlockTxPlugin>();
    APP.plugin_init::<L2TxBatchPlugin>();
    APP.plugin_init::<L2StateBatchPlugin>();
    APP.plugin_init::<L2TxReceiptPlugin>();
    APP.plugin_init::<L2EnqueuePlugin>();
    APP.plugin_init::<L1TxLogPlugin>();
    APP.plugin_init::<TaskPlugin>();
    APP.startup();
    APP.execute();
}
