use appbase::prelude::*;
use clap::Arg;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::error::error::ExpectedError;
use crate::libs;
use crate::libs::convert::hex_to_decimal_converter;
use crate::libs::opt::opt_to_result;
use crate::libs::request;
use crate::libs::serde::{get_array, get_object, get_string};
use crate::libs::subscribe::task_loader;
use crate::message;
use crate::plugin::l2_tx_receipt::{L2TxReceiptMsg, L2TxReceiptPlugin};
use crate::plugin::postgres::{PostgresMsg, PostgresPlugin};
use crate::plugin::rocks::RocksPlugin;
use crate::plugin::slack::SlackPlugin;
use crate::types::channel::MultiSender;
use crate::types::subscribe::SubscribeEvent;

#[appbase_plugin(RocksPlugin, PostgresPlugin, SlackPlugin, L2TxReceiptPlugin)]
pub struct L2BlockTxPlugin {
    sub_event: Option<SubscribeEvent>,
    senders: Option<MultiSender>,
    receiver: Option<Receiver>,
}

const CHAIN: &str = "optimism";
const TASK_PREFIX: &str = "task:optimism";
const TASK_NAME: &str = "l2_block_tx";
const TASK_FILE: &str = "task/l2_block_tx.json";
const DEFAULT_POLL_INTERVAL: u64 = 100;

message!(L2BlockTxMsg; {method: String});

impl Plugin for L2BlockTxPlugin {
    fn new() -> Self {
        APP.options.arg(Arg::new("l2blocktx::poll-interval").long("l2blocktx-poll-interval").takes_value(true));
        L2BlockTxPlugin {
            sub_event: None,
            senders: None,
            receiver: None,
        }
    }

    fn init(&mut self) {
        let senders = MultiSender::new(vec!("rocks", "postgres", "slack", "l2_tx_receipt" /*"elasticsearch"*/));
        self.senders = Some(senders.to_owned());
        self.receiver = Some(APP.channels.subscribe(TASK_NAME));
        let rocksdb = APP.run_with::<RocksPlugin, _, _>(|rocks| rocks.get_db());
        self.sub_event = Some(task_loader(rocksdb, TASK_FILE, CHAIN, TASK_PREFIX, TASK_NAME).expect(format!("failed to load task! task={}", TASK_NAME).as_str()));
    }

    fn startup(&mut self) {
        let receiver = self.receiver.take().unwrap();
        let sub_event = self.sub_event.take().unwrap();
        let senders = self.senders.take().unwrap();
        let app = APP.quit_handle().unwrap();

        Self::recv(receiver, sub_event, senders, app);
    }

    fn shutdown(&mut self) {}
}

impl L2BlockTxPlugin {
    fn recv(mut receiver: Receiver, mut sub_event: SubscribeEvent, senders: MultiSender, app: QuitHandle) {
        APP.spawn(async move {
            if let Ok(message) = receiver.try_recv() {
                if let Err(err) = libs::subscribe::message_handler(message, &mut sub_event, &senders) {
                    let _ = libs::error::warn_handler(senders.get("slack"), err);
                }
            }
            if sub_event.is_workable() {
                match Self::event_handler(&sub_event, &senders).await {
                    Ok(_) => {
                        libs::subscribe::task_syncer(&sub_event, &senders);
                        sub_event.next_idx();
                    }
                    Err(err) => libs::subscribe::error_handler(err, &mut sub_event, &senders)
                }
            }
            if !app.is_quitting() {
                let poll_interval = libs::opt::get_value::<u64>("l2blocktx::poll-interval").unwrap_or(DEFAULT_POLL_INTERVAL);
                tokio::time::sleep(tokio::time::Duration::from_millis(poll_interval)).await;
                Self::recv(receiver, sub_event, senders, app);
            }
        });
    }

    async fn event_handler(sub_event: &SubscribeEvent, senders: &MultiSender) -> Result<(), ExpectedError> {
        let req_url = sub_event.active_node();
        let hex_idx = format!("0x{:x}", sub_event.curr_idx);
        let req_body = json!({
                    "jsonrpc": "2.0",
                    "method": "eth_getBlockByNumber",
                    "params": [ hex_idx, true ],
                    "id": 1
                });
        let response = request::post_async(req_url.as_str(), req_body.to_string().as_str()).await?;
        let _ = libs::subscribe::response_verifier(&response, TASK_NAME, "result", sub_event.get_filter())?;
        let block = get_object(&response, "result")?;
        let converted_block = hex_to_decimal_converter(block, vec!["number", "size", "timestamp", "gasLimit", "gasUsed"])?;
        let pg_sender = senders.get("postgres");
        let _ = pg_sender.send(PostgresMsg::new(String::from("optimism_blocks"), Value::Object(converted_block.to_owned())))?;
        let txs = get_array(&block, "transactions")?;

        let receipt_sender = senders.get("l2_tx_receipt");
        for tx in txs.iter() {
            let tx_map = opt_to_result(tx.as_object())?;
            let converted_tx = hex_to_decimal_converter(tx_map, vec!["blockNumber", "gas", "gasPrice", "nonce", "transactionIndex", "value", "l1BlockNumber", "l1Timestamp", "index", "queueIndex"])?;
            let tx_hash = get_string(&converted_tx, "hash")?;
            let _ = receipt_sender.send(L2TxReceiptMsg::new(tx_hash))?;
            let _ = pg_sender.send(PostgresMsg::new(String::from("optimism_block_txs"), Value::Object(converted_tx.to_owned())))?;
        }
        Ok(())
    }
}