use std::collections::HashMap;

use appbase::prelude::*;
use clap::Arg;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::error::error::ExpectedError;
use crate::libs;
use crate::libs::convert::hex_to_decimal_converter;
use crate::libs::opt::opt_to_result;
use crate::libs::request;
use crate::libs::serde::{get_array, get_object, get_str};
use crate::libs::subscribe::{load_retry_queue, load_task_from_json, remove_from_retry_queue, save_retry_queue};
use crate::message;
use crate::plugin::postgres::{PostgresMsg, PostgresPlugin};
use crate::plugin::rocks::RocksPlugin;
use crate::plugin::slack::SlackPlugin;
use crate::types::channel::MultiSender;
use crate::types::subscribe::{RetryJob, SubscribeEvent};

#[appbase_plugin(RocksPlugin, PostgresPlugin, SlackPlugin)]
pub struct L2TxReceiptPlugin {
    sub_event: Option<SubscribeEvent>,
    senders: Option<MultiSender>,
    receiver: Option<Receiver>,
    retry_queue: Option<HashMap<String, L2TxReceiptRetryJob>>,
}

const CHAIN: &str = "optimism";
const TASK_PREFIX: &str = "task:optimism";
const TASK_NAME: &str = "l2_tx_receipt";
const TASK_FILE: &str = "task/l2_tx_receipt.json";
const RETRY_PREFIX: &str = "retry:optimism:l2_tx_receipt";
const DEFAULT_RETRY_COUNT: u32 = 3;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct L2TxReceiptRetryJob {
    retry_id: String,
    tx_hash: String,
    retry_count: u32,
}

impl RetryJob for L2TxReceiptRetryJob {
    fn get_retry_id(&self) -> String { self.retry_id.clone() }
    fn get_retry_count(&self) -> u32 { self.retry_count }
    fn decrease_retry_count(&mut self) { self.retry_count -= 1; }
    fn is_retry_available(&self) -> bool { self.retry_count > 0 }
}

impl L2TxReceiptRetryJob {
    fn new(tx_hash: String, retry_count: u32) -> Self {
        Self {
            retry_id: format!("{}:{}", RETRY_PREFIX, tx_hash),
            tx_hash,
            retry_count,
        }
    }
}

message!(L2TxReceiptMsg; {tx_hash: String});

impl Plugin for L2TxReceiptPlugin {
    fn new() -> Self {
        APP.options.arg(Arg::new("l2txreceipt::retry-count").long("l2txreceipt-retry-count").takes_value(true));
        L2TxReceiptPlugin {
            sub_event: None,
            senders: None,
            receiver: None,
            retry_queue: None,
        }
    }

    fn init(&mut self) {
        let senders = MultiSender::new(vec!("postgres", "slack" /*"elasticsearch"*/));
        self.senders = Some(senders.to_owned());
        self.receiver = Some(APP.channels.subscribe(TASK_NAME));
        self.sub_event = Some(load_task_from_json(TASK_FILE, CHAIN, TASK_PREFIX, TASK_NAME).expect(format!("failed to load task! task={}", TASK_NAME).as_str()));
        let rocksdb = APP.run_with::<RocksPlugin, _, _>(|rocks| rocks.get_db());
        self.retry_queue = Some(load_retry_queue::<L2TxReceiptRetryJob>(rocksdb, RETRY_PREFIX).expect(format!("failed to load retry queue! task={}", TASK_NAME).as_str()))
    }

    fn startup(&mut self) {
        let receiver = self.receiver.take().unwrap();
        let senders = self.senders.take().unwrap();
        let sub_event = self.sub_event.take().unwrap();
        let retry_queue = self.retry_queue.take().unwrap();
        let app = APP.quit_handle().unwrap();

        Self::recv(receiver, sub_event, senders, retry_queue, app);
    }

    fn shutdown(&mut self) {}
}

impl L2TxReceiptPlugin {
    fn recv(mut receiver: Receiver, sub_event: SubscribeEvent, senders: MultiSender, mut retry_queue: HashMap<String, L2TxReceiptRetryJob>, app: QuitHandle) {
        APP.spawn_blocking(move || {
            if let Ok(message) = receiver.try_recv() {
                if let Err(err) = Self::message_handler(message, &sub_event, &senders, &mut retry_queue) {
                    let _ = libs::error::error_handler(senders.get("slack"), err);
                }
            }
            if let Err(err) = Self::retry_handler(&mut retry_queue, &sub_event, &senders) {
                let _ = libs::error::error_handler(senders.get("slack"), err);
            }
            if !app.is_quitting() {
                Self::recv(receiver, sub_event, senders, retry_queue, app);
            }
        });
    }

    fn message_handler(message: Value, sub_event: &SubscribeEvent, senders: &MultiSender, retry_queue: &mut HashMap<String, L2TxReceiptRetryJob>) -> Result<(), ExpectedError> {
        let parsed_msg = message.as_object().unwrap();
        let tx_hash = get_str(parsed_msg, "tx_hash")?;
        if let Err(err) = Self::receipt_syncer(tx_hash, sub_event, senders) {
            let retry_count = libs::opt::get_value::<u32>("l1txlog::retry-count").unwrap_or(DEFAULT_RETRY_COUNT);
            let retry_job = L2TxReceiptRetryJob::new(String::from(tx_hash), retry_count);
            retry_queue.insert(retry_job.get_retry_id(), retry_job.clone());
            let rocks_sender = senders.get("rocks");
            let _ = save_retry_queue(&rocks_sender, retry_job.get_retry_id(), Value::String(json!(retry_job).to_string()))?;
            return Err(err);
        }
        Ok(())
    }

    fn receipt_syncer(tx_hash: &str, sub_event: &SubscribeEvent, senders: &MultiSender) -> Result<(), ExpectedError> {
        let req_url = sub_event.active_node();
        let req_body = json!({
                    "jsonrpc": "2.0",
                    "method": "eth_getTransactionReceipt",
                    "params": [ tx_hash ],
                    "id": 1
                });
        let response = request::post(req_url.as_str(), req_body.to_string().as_str())?;
        if !libs::subscribe::is_value_created(&response, "result") {
            return Err(ExpectedError::NoneError(format!("receipt does not created...tx_hash={}", tx_hash)));
        }
        let receipt = get_object(&response, "result")?;
        let converted_receipt = hex_to_decimal_converter(receipt, vec!["blockNumber", "cumulativeGasUsed", "gasUsed", "status", "transactionIndex"])?;
        let pg_sender = senders.get("postgres");
        let _ = pg_sender.send(PostgresMsg::new(String::from("optimism_tx_receipts"), Value::Object(converted_receipt.to_owned())))?;
        let logs = get_array(&receipt, "logs")?;
        for log in logs.iter() {
            let log_map = opt_to_result(log.as_object())?;
            let converted_log = hex_to_decimal_converter(log_map, vec!["blockNumber", "transactionIndex", "logIndex"])?;
            let _ = pg_sender.send(PostgresMsg::new(String::from("optimism_tx_receipt_logs"), Value::Object(converted_log.to_owned())))?;
        }
        Ok(())
    }

    fn retry_handler(retry_queue: &mut HashMap<String, L2TxReceiptRetryJob>, sub_event: &SubscribeEvent, senders: &MultiSender) -> Result<(), ExpectedError> {
        let mut remove_job = Vec::new();
        let rocks_sender = senders.get("rocks");
        if !retry_queue.is_empty() {
            for (retry_id, retry_job) in retry_queue.iter_mut() {
                if !retry_job.is_retry_available() || Self::receipt_syncer(&retry_job.tx_hash, sub_event, senders).is_ok() {
                    remove_job.push(retry_id.clone());
                    let _ = remove_from_retry_queue(&rocks_sender, retry_id.clone())?;
                } else {
                    retry_job.decrease_retry_count();
                    let _ = save_retry_queue(&rocks_sender, retry_job.get_retry_id(), retry_job)?;
                }
            }
            if !remove_job.is_empty() {
                for retry_id in remove_job {
                    retry_queue.remove(&retry_id);
                }
            }
        }
        Ok(())
    }
}