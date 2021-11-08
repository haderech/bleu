use std::thread;
use std::time::Duration;

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
use crate::types::channel::MultiSender;
use crate::types::subscribe::SubscribeEvent;

#[appbase_plugin(RocksPlugin, PostgresPlugin)]
pub struct L2TxBatchPlugin {
    sub_event: Option<SubscribeEvent>,
    senders: Option<MultiSender>,
    receiver: Option<Receiver>,
    poll_interval: Option<u64>,
}

const CHAIN: &str = "optimism";
const TASK_PREFIX: &str = "task:optimism";
const TASK_NAME: &str = "l2_tx_batch";
const TASK_FILE: &str = "task/l2_tx_batch.json";

message!(L2TxBatchMsg; {method: String});

impl Plugin for L2TxBatchPlugin {
    fn new() -> Self {
        APP.options.arg(Arg::new("l2txbatch::poll-interval").long("l2txbatch-poll-interval").takes_value(true));
        L2TxBatchPlugin {
            sub_event: None,
            senders: None,
            receiver: None,
            poll_interval: None,
        }
    }

    fn init(&mut self) {
        let senders = MultiSender::new(vec!("rocks", "postgres" /*"elasticsearch"*/));
        self.senders = Some(senders.to_owned());
        self.receiver = Some(APP.channels.subscribe(TASK_NAME));
        let rocksdb = APP.run_with::<RocksPlugin, _, _>(|rocks| rocks.get_db());
        self.sub_event = Some(task_loader(rocksdb, TASK_FILE, CHAIN, TASK_PREFIX, TASK_NAME).expect(format!("failed to load task! task={}", TASK_NAME).as_str()));
        self.poll_interval = Some(libs::opt::get_value("l2txbatch::poll-interval").unwrap());
    }

    fn startup(&mut self) {
        let receiver = self.receiver.take().unwrap();
        let sub_event = self.sub_event.take().unwrap();
        let senders = self.senders.take().unwrap();
        let poll_interval = self.poll_interval.take().unwrap();
        let app = APP.quit_handle().unwrap();

        Self::recv(receiver, sub_event, senders, poll_interval, app);
    }

    fn shutdown(&mut self) {}
}

impl L2TxBatchPlugin {
    fn recv(mut receiver: Receiver, mut sub_event: SubscribeEvent, senders: MultiSender, poll_interval: u64, app: QuitHandle) {
        APP.spawn_blocking(move || {
            if let Ok(message) = receiver.try_recv() {
                libs::subscribe::message_handler(message, &mut sub_event, &senders);
            }
            if sub_event.is_workable() {
                match Self::event_handler(&sub_event, &senders) {
                    Ok(_) => {
                        libs::subscribe::task_syncer(&sub_event, &senders);
                        sub_event.next_idx();
                    }
                    Err(err) => libs::subscribe::error_handler(err, &mut sub_event, &senders)
                }
            }
            if !app.is_quitting() {
                thread::sleep(Duration::from_millis(poll_interval));
                Self::recv(receiver, sub_event, senders, poll_interval, app);
            }
        });
    }

    fn event_handler(sub_event: &SubscribeEvent, senders: &MultiSender) -> Result<(), ExpectedError> {
        let req_url = libs::subscribe::create_req_url(sub_event.active_node(), sub_event.curr_idx);
        let response = request::get(req_url.as_str())?;
        let _ = libs::subscribe::response_verifier(&response, TASK_NAME, "batch", sub_event.get_filter())?;
        let batch = get_object(&response, "batch")?;
        let converted_batch = number_to_string_convert(batch, vec!["index", "timestamp", "size", "blockNumber", "prevTotalElements"])?;
        let pg_sender = senders.get("postgres");
        let _ = pg_sender.send(PostgresMsg::new(String::from("optimism_tx_batches"), Value::Object(converted_batch.to_owned())))?;
        let txs = get_array(&response, "transactions")?;
        for tx in txs.iter() {
            let tx_map = opt_to_result(tx.as_object())?;
            let converted_tx = number_to_string_convert(tx_map, vec!["index", "batchIndex", "blockNumber", "timestamp", "queueIndex"])?;
            let _ = pg_sender.send(PostgresMsg::new(String::from("optimism_txs"), Value::Object(converted_tx.to_owned())))?;
        }
        Ok(())
    }
}