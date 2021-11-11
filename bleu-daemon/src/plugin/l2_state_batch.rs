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
use crate::plugin::slack::SlackPlugin;
use crate::types::channel::MultiSender;
use crate::types::subscribe::SubscribeEvent;

#[appbase_plugin(RocksPlugin, PostgresPlugin, SlackPlugin)]
pub struct L2StateBatchPlugin {
    sub_event: Option<SubscribeEvent>,
    senders: Option<MultiSender>,
    receiver: Option<Receiver>,
}

const CHAIN: &str = "optimism";
const TASK_PREFIX: &str = "task:optimism";
const TASK_NAME: &str = "l2_state_batch";
const TASK_FILE: &str = "task/l2_state_batch.json";
const DEFAULT_POLL_INTERVAL: u64 = 1000;

message!(L2StateBatchMsg; {method: String});

impl Plugin for L2StateBatchPlugin {
    fn new() -> Self {
        APP.options.arg(Arg::new("l2statebatch::poll-interval").long("l2statebatch-poll-interval").takes_value(true));
        L2StateBatchPlugin {
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

impl L2StateBatchPlugin {
    fn recv(mut receiver: Receiver, mut sub_event: SubscribeEvent, senders: MultiSender, app: QuitHandle) {
        APP.spawn_blocking(move || {
            if let Ok(message) = receiver.try_recv() {
                if let Err(err) = libs::subscribe::message_handler(message, &mut sub_event, &senders) {
                    let _ = libs::error::warn_handler(senders.get("slack"), err);
                }
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
                let poll_interval = libs::opt::get_value::<u64>("l2statebatch::poll-interval").unwrap_or(DEFAULT_POLL_INTERVAL);
                thread::sleep(Duration::from_millis(poll_interval));
                Self::recv(receiver, sub_event, senders, app);
            }
        });
    }

    fn event_handler(sub_event: &SubscribeEvent, senders: &MultiSender) -> Result<(), ExpectedError> {
        let req_url = libs::subscribe::create_req_url(sub_event.active_node(), sub_event.curr_idx);
        let response = request::get(req_url.as_str())?;
        let _ = libs::subscribe::response_verifier(&response, TASK_NAME, "batch", sub_event.get_filter())?;
        let batch = get_object(&response, "batch")?;
        let converted_batch = number_to_string_convert(batch, vec!["index", "blockNumber", "timestamp", "size", "prevTotalElements"])?;
        let pg_sender = senders.get("postgres");
        let _ = pg_sender.send(PostgresMsg::new(String::from("optimism_state_batches"), Value::Object(batch.clone())))?;

        let state_roots = get_array(&response, "stateRoots")?;
        let l1_tx_hash = opt_to_result(converted_batch.get("l1TransactionHash"))?;
        for state_root in state_roots.iter() {
            let state_root_map = opt_to_result(state_root.as_object())?;
            let mut converted_state_root = number_to_string_convert(state_root_map, vec!["index", "batchIndex"])?;
            converted_state_root.insert(String::from("l1_tx_hash"), l1_tx_hash.clone());
            let _ = pg_sender.send(PostgresMsg::new(String::from("optimism_state_roots"), Value::Object(converted_state_root)))?;
        }
        Ok(())
    }
}