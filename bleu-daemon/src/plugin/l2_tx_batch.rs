use appbase::prelude::*;
use clap::Arg;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::error::error::ExpectedError;
use crate::libs;
use crate::libs::convert::number_to_string_convert;
use crate::libs::opt::opt_to_result;
use crate::libs::request;
use crate::libs::serde::{get_array, get_object};
use crate::libs::subscribe::task_loader;
use crate::message;
use crate::plugin::postgres::{PostgresMsg, PostgresPlugin};
use crate::plugin::rocks::RocksPlugin;
use crate::plugin::slack::SlackPlugin;
use crate::types::channel::MultiSender;
use crate::types::subscribe::SubscribeEvent;

#[appbase_plugin(RocksPlugin, PostgresPlugin, SlackPlugin)]
pub struct L2TxBatchPlugin {
    sub_event: Option<SubscribeEvent>,
    senders: Option<MultiSender>,
    receiver: Option<Receiver>,
}

const CHAIN: &str = "optimism";
const TASK_PREFIX: &str = "task:optimism";
const TASK_NAME: &str = "l2_tx_batch";
const TASK_FILE: &str = "task/l2_tx_batch.json";
const DEFAULT_POLL_INTERVAL: u64 = 1000;

message!(L2TxBatchMsg; {method: String});

impl Plugin for L2TxBatchPlugin {
    fn new() -> Self {
        APP.options.arg(Arg::new("l2txbatch::poll-interval").long("l2txbatch-poll-interval").takes_value(true));
        L2TxBatchPlugin {
            sub_event: None,
            senders: None,
            receiver: None,
        }
    }

    fn init(&mut self) {
        let senders = MultiSender::new(vec!("rocks", "postgres", "slack" /*"elasticsearch"*/));
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

impl L2TxBatchPlugin {
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
                let poll_interval = libs::opt::get_value::<u64>("l2txbatch::poll-interval").unwrap_or(DEFAULT_POLL_INTERVAL);
                tokio::time::sleep(tokio::time::Duration::from_millis(poll_interval)).await;
                Self::recv(receiver, sub_event, senders, app);
            }
        });
    }

    async fn event_handler(sub_event: &SubscribeEvent, senders: &MultiSender) -> Result<(), ExpectedError> {
        let req_url = libs::subscribe::create_req_url(sub_event.active_node(), sub_event.curr_idx);
        let response = request::get_async(req_url.as_str()).await?;
        let _ = libs::subscribe::response_verifier(&response, TASK_NAME, "batch", sub_event.get_filter())?;
        let batch = get_object(&response, "batch")?;
        let converted_batch = number_to_string_convert(batch, vec!["index", "timestamp", "size", "blockNumber", "prevTotalElements"])?;
        let pg_sender = senders.get("postgres");
        let _ = pg_sender.send(PostgresMsg::new(String::from("optimism_tx_batches"), Value::Object(converted_batch.clone())))?;
        let txs = get_array(&response, "transactions")?;
        let l1_tx_hash = opt_to_result(converted_batch.get("l1TransactionHash"))?;

        for tx in txs.iter() {
            let tx_map = opt_to_result(tx.as_object())?;
            let mut converted_tx = number_to_string_convert(tx_map, vec!["index", "batchIndex", "blockNumber", "timestamp", "queueIndex"])?;
            converted_tx.insert(String::from("l1_tx_hash"), l1_tx_hash.clone());
            let _ = pg_sender.send(PostgresMsg::new(String::from("optimism_txs"), Value::Object(converted_tx.to_owned())))?;
        }
        Ok(())
    }
}