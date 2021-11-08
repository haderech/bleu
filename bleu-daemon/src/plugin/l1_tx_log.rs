use std::collections::HashMap;
use std::fs::File;

use appbase::prelude::*;
use ethabi::{Contract, RawLog};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::error::error::ExpectedError;
use crate::libs;
use crate::libs::convert::hex_to_decimal_converter;
use crate::libs::opt::opt_to_result;
use crate::libs::request;
use crate::libs::serde::{get_array, get_str, get_u64};
use crate::libs::subscribe::{load_retry_queue, load_task_from_json, remove_from_retry_queue, save_retry_queue};
use crate::message;
use crate::plugin::postgres::{PostgresMsg, PostgresPlugin};
use crate::plugin::rocks::RocksPlugin;
use crate::types::channel::MultiSender;
use crate::types::subscribe::{RetryJob, SubscribeEvent};

#[appbase_plugin(RocksPlugin, PostgresPlugin)]
pub struct L1TxLogPlugin {
    sub_event: Option<SubscribeEvent>,
    senders: Option<MultiSender>,
    receiver: Option<Receiver>,
    retry_queue: Option<HashMap<String, L1TxLogRetryJob>>,
}

const CHAIN: &str = "ethereum";
const TASK_PREFIX: &str = "task:ethereum";
const TASK_NAME: &str = "l1_tx_log";
const TASK_FILE: &str = "task/l1_tx_log.json";
const ABI_FILE: &str = "abi/tx_enqueued.json";
const EVENT: &str = "TransactionEnqueued";
const TOPIC0: &str = "0x4b388aecf9fa6cc92253704e5975a6129a4f735bdbd99567df4ed0094ee4ceb5";
const RETRY_PREFIX: &str = "retry:ethereum:l1_tx_log";

#[derive(Debug, Clone, Deserialize, Serialize)]
struct L1TxLogRetryJob {
    retry_id: String,
    block_number: u64,
    queue_index: u64,
}

impl RetryJob for L1TxLogRetryJob {
    fn get_retry_id(&self) -> String { self.retry_id.clone() }
}

impl L1TxLogRetryJob {
    fn new(block_number: u64, queue_index: u64) -> Self {
        Self {
            retry_id: format!("{}:{}:{}", RETRY_PREFIX, block_number, queue_index),
            block_number,
            queue_index,
        }
    }
}

type LogKeyValue = HashMap<String, String>;

message!(L1TxLogMsg; {block_number: u64}, {queue_index: u64});

impl Plugin for L1TxLogPlugin {
    fn new() -> Self {
        L1TxLogPlugin {
            sub_event: None,
            senders: None,
            receiver: None,
            retry_queue: None,
        }
    }

    fn init(&mut self) {
        let senders = MultiSender::new(vec!("rocks", "postgres" /*"elasticsearch"*/));
        self.senders = Some(senders.to_owned());
        self.receiver = Some(APP.channels.subscribe(TASK_NAME));
        self.sub_event = Some(load_task_from_json(TASK_FILE, CHAIN, TASK_PREFIX, TASK_NAME).expect(format!("failed to load task! task={}", TASK_NAME).as_str()));
        let rocksdb = APP.run_with::<RocksPlugin, _, _>(|rocks| rocks.get_db());
        self.retry_queue = Some(load_retry_queue::<L1TxLogRetryJob>(rocksdb, RETRY_PREFIX).expect(format!("failed to load retry queue! task={}", TASK_NAME).as_str()));
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

impl L1TxLogPlugin {
    fn recv(mut receiver: Receiver, sub_event: SubscribeEvent, senders: MultiSender, mut retry_queue: HashMap<String, L1TxLogRetryJob>, app: QuitHandle) {
        APP.spawn_blocking(move || {
            if let Ok(message) = receiver.try_recv() {
                if let Err(err) = Self::message_handler(message.clone(), &sub_event, &senders, &mut retry_queue) {
                    log::error!("{}, message={:?}", err.to_string(), message);
                }
            }
            if let Err(err) = Self::retry_handler(&mut retry_queue, &sub_event, &senders) {
                log::error!("{}", err.to_string());
            }

            if !app.is_quitting() {
                Self::recv(receiver, sub_event, senders, retry_queue, app);
            }
        });
    }

    fn message_handler(message: Value, sub_event: &SubscribeEvent, senders: &MultiSender, retry_queue: &mut HashMap<String, L1TxLogRetryJob>) -> Result<(), ExpectedError> {
        let parsed_msg = opt_to_result(message.as_object())?;
        let block_number = get_u64(parsed_msg, "block_number")?;
        let queue_index = get_u64(parsed_msg, "queue_index")?;

        match Self::log_filter(block_number, queue_index, sub_event) {
            Ok(log_value) => {
                let pg_sender = senders.get("postgres");
                let _ = Self::save_log(log_value, queue_index, pg_sender)?;
                Ok(())
            }
            Err(err) => {
                let retry_job = L1TxLogRetryJob::new(block_number, queue_index);
                retry_queue.insert(retry_job.get_retry_id(), retry_job.clone());
                let rocks_sender = senders.get("rocks");
                let _ = save_retry_queue(rocks_sender, retry_job.get_retry_id(), Value::String(json!(retry_job).to_string()))?;
                Err(err)
            }
        }
    }

    fn is_matched_log(abi_file: &str, event_name: &str, log_value: &Value, queue_index: String) -> Result<bool, ExpectedError> {
        let log_map = opt_to_result(log_value.as_object())?;
        let data = get_str(log_map, "data")?;
        let log_kv = Self::log_parser(abi_file, event_name, data)?;
        let log_queue_idx = opt_to_result(log_kv.get("_queueIndex"))?;
        Ok(log_queue_idx.clone() == queue_index)
    }

    fn log_parser(abi_file: &str, event_name: &str, data: &str) -> Result<LogKeyValue, ExpectedError> {
        let contract = Contract::load(File::open(abi_file)?)?;
        let event = contract.event(event_name)?;
        let topic_hash = event.signature();
        let prefix_removed_data = data.trim_start_matches("0x");
        let raw_log = RawLog {
            topics: vec![topic_hash],
            data: hex::decode(prefix_removed_data)?,
        };
        let parsed_log = event.parse_log(raw_log)?;
        let log_params = parsed_log.params;
        let mut log_map = HashMap::new();
        for log_param in log_params {
            let name = log_param.name;
            let value = log_param.value;
            log_map.insert(name, value.to_string());
        }
        Ok(log_map)
    }

    fn log_filter(block_number: u64, queue_index: u64, sub_event: &SubscribeEvent) -> Result<Value, ExpectedError> {
        let block_number_hex = format!("0x{:x}", block_number);
        let queue_index_hex = format!("{:x}", queue_index);
        let req_url = sub_event.active_node();
        let req_body = json!({
                    "jsonrpc": "2.0",
                    "method": "eth_getLogs",
                    "params": [{
                        "fromBlock": block_number_hex,
                        "toBlock": block_number_hex,
                        "topics": [ TOPIC0 ]
                    }],
                    "id": 1
                });
        let response = request::post(req_url.as_str(), req_body.to_string().as_str())?;
        if !libs::subscribe::is_log_created(&response, "result") {
            return Err(ExpectedError::NoneError(format!("logs does not created! response={:?}, block_number={}, topic={}", response, block_number, TOPIC0)));
        }
        let logs = get_array(&response, "result")?;
        for log_value in logs {
            let is_matched = Self::is_matched_log(ABI_FILE, EVENT, log_value, queue_index_hex.clone());
            if is_matched.is_ok() && is_matched.unwrap() {
                return Ok(log_value.to_owned());
            }
        }
        Err(ExpectedError::NoneError(format!("matched log does not exist! block_number={}, queue_index={}, topic={}", block_number, queue_index, TOPIC0)))
    }

    fn retry_handler(retry_queue: &mut HashMap<String, L1TxLogRetryJob>, sub_event: &SubscribeEvent, senders: &MultiSender) -> Result<(), ExpectedError> {
        let mut success_job = Vec::new();
        if !retry_queue.is_empty() {
            for (retry_id, retry_job) in retry_queue.iter() {
                if let Ok(()) = Self::retry(&retry_job, &sub_event, &senders) {
                    success_job.push(retry_id.clone());
                    let rocks_sender = senders.get("rocks");
                    let _ = remove_from_retry_queue(rocks_sender, retry_id.clone());
                }
            }
        }
        if !success_job.is_empty() {
            for retry_id in success_job {
                retry_queue.remove(&retry_id);
            }
        }
        Ok(())
    }

    fn retry(retry_job: &L1TxLogRetryJob, sub_event: &SubscribeEvent, senders: &MultiSender) -> Result<(), ExpectedError> {
        let log_value = Self::log_filter(retry_job.block_number, retry_job.queue_index, sub_event)?;
        let pg_sender = senders.get("postgres");
        let _ = Self::save_log(log_value, retry_job.queue_index, pg_sender)?;
        Ok(())
    }

    fn save_log(log_value: Value, queue_index: u64, pg_sender: Sender) -> Result<(), ExpectedError> {
        let mut converted_log = hex_to_decimal_converter(opt_to_result(log_value.as_object())?, vec!["blockNumber", "logIndex", "transactionIndex"])?;
        converted_log.insert(String::from("queue_index"), Value::String(queue_index.to_string()));
        let _ = pg_sender.send(PostgresMsg::new(String::from("ethereum_tx_logs"), Value::Object(converted_log)));
        Ok(())
    }
}