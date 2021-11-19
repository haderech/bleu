use std::collections::HashMap;
use std::fs::File;

use appbase::prelude::*;
use clap::Arg;
use ethabi::{Contract, RawLog};
use jsonrpc_core::Params;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::{libs, validation};
use crate::error::error::ExpectedError;
use crate::libs::convert::hex_to_decimal_converter;
use crate::libs::opt::opt_to_result;
use crate::libs::request;
use crate::libs::serde::{get_array, get_str, get_u64};
use crate::libs::subscribe::{load_retry_queue, load_task_from_json, remove_from_retry_queue, save_retry_queue};
use crate::message;
use crate::plugin::jsonrpc::JsonRpcPlugin;
use crate::plugin::postgres::{PostgresMsg, PostgresPlugin};
use crate::plugin::rocks::RocksPlugin;
use crate::plugin::slack::SlackPlugin;
use crate::types::channel::MultiSender;
use crate::types::subscribe::{RetryJob, SubscribeEvent};

#[appbase_plugin(RocksPlugin, PostgresPlugin, SlackPlugin)]
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
const DEFAULT_RETRY_COUNT: u32 = 3;

#[derive(Debug, Clone, Deserialize, Serialize)]
struct L1TxLogRetryJob {
    retry_id: String,
    block_number: u64,
    queue_index: u64,
    retry_count: u32,
}

impl RetryJob for L1TxLogRetryJob {
    fn get_retry_id(&self) -> String { self.retry_id.clone() }
    fn get_retry_count(&self) -> u32 { self.retry_count }
    fn decrease_retry_count(&mut self) { self.retry_count -= 1; }
    fn is_retry_available(&self) -> bool { self.retry_count > 0 }
}

impl L1TxLogRetryJob {
    fn new(block_number: u64, queue_index: u64, retry_count: u32) -> Self {
        Self {
            retry_id: format!("{}:{}:{}", RETRY_PREFIX, block_number, queue_index),
            block_number,
            queue_index,
            retry_count,
        }
    }
}

type LogKeyValue = HashMap<String, String>;

message!(L1TxLogMsg; {block_number: u64}, {queue_index: u64});

impl Plugin for L1TxLogPlugin {
    fn new() -> Self {
        APP.options.arg(Arg::new("l1txlog::retry-count").long("l1txlog-retry-count").takes_value(true));
        L1TxLogPlugin {
            sub_event: None,
            senders: None,
            receiver: None,
            retry_queue: None,
        }
    }

    fn init(&mut self) {
        let senders = MultiSender::new(vec!("rocks", "postgres", "slack", "l1_tx_log"));
        self.senders = Some(senders.to_owned());
        self.receiver = Some(APP.channels.subscribe(TASK_NAME));
        self.sub_event = Some(load_task_from_json(TASK_FILE, CHAIN, TASK_PREFIX, TASK_NAME).expect(format!("failed to load task! task={}", TASK_NAME).as_str()));
        let rocksdb = APP.run_with::<RocksPlugin, _, _>(|rocks| rocks.get_db());
        self.retry_queue = Some(load_retry_queue::<L1TxLogRetryJob>(rocksdb, RETRY_PREFIX).expect(format!("failed to load retry queue! task={}", TASK_NAME).as_str()));
        self.jsonrpc_register();
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

    fn message_handler(message: Value, sub_event: &SubscribeEvent, senders: &MultiSender, retry_queue: &mut HashMap<String, L1TxLogRetryJob>) -> Result<(), ExpectedError> {
        let parsed_msg = opt_to_result(message.as_object())?;
        let block_number = get_u64(parsed_msg, "block_number")?;
        let queue_index = get_u64(parsed_msg, "queue_index")?;

        let pg_sender = senders.get("postgres");
        if let Err(err) = Self::log_syncer(block_number, queue_index, sub_event, &pg_sender) {
            let retry_count = libs::opt::get_value::<u32>("l1txlog::retry-count").unwrap_or(DEFAULT_RETRY_COUNT);
            let retry_job = L1TxLogRetryJob::new(block_number, queue_index, retry_count);
            retry_queue.insert(retry_job.get_retry_id(), retry_job.clone());
            let rocks_sender = senders.get("rocks");
            let _ = save_retry_queue(&rocks_sender, retry_job.get_retry_id(), retry_job)?;
            return Err(err);
        }
        Ok(())
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

    fn log_syncer(block_number: u64, queue_index: u64, sub_event: &SubscribeEvent, pg_sender: &Sender) -> Result<(), ExpectedError> {
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
                let mut converted_log = hex_to_decimal_converter(opt_to_result(log_value.as_object())?, vec!["blockNumber", "logIndex", "transactionIndex"])?;
                converted_log.insert(String::from("queue_index"), Value::String(queue_index.to_string()));
                let _ = pg_sender.send(PostgresMsg::new(String::from("ethereum_tx_logs"), Value::Object(converted_log)));
                return Ok(());
            }
        }
        Err(ExpectedError::NoneError(format!("matched log does not exist! block_number={}, queue_index={}, topic={}", block_number, queue_index, TOPIC0)))
    }

    fn retry_handler(retry_queue: &mut HashMap<String, L1TxLogRetryJob>, sub_event: &SubscribeEvent, senders: &MultiSender) -> Result<(), ExpectedError> {
        let mut remove_job = Vec::new();
        let rocks_sender = senders.get("rocks");
        if !retry_queue.is_empty() {
            for (retry_id, retry_job) in retry_queue.iter_mut() {
                let pg_sender = senders.get("postgres");
                if !retry_job.is_retry_available() || Self::log_syncer(retry_job.block_number, retry_job.queue_index, sub_event, &pg_sender).is_ok() {
                    remove_job.push(retry_id.clone());
                    let _ = remove_from_retry_queue(&rocks_sender, retry_id.clone());
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

    fn jsonrpc_register(&self) {
        let senders = self.senders.as_ref().unwrap();
        let self_sender = senders.get(TASK_NAME);

        APP.run_with::<JsonRpcPlugin, _, _>(|jsonrpc| {
            jsonrpc.add_method(String::from("retry_l1_tx_log"), move |params: Params| {
                let response = match Self::request_handler(params, &self_sender) {
                    Ok(response) => response,
                    Err(err) => json!({"error": err.to_string()}),
                };
                Box::new(futures::future::ok(response))
            });
        });
    }

    fn request_handler(params: Params, self_sender: &Sender) -> Result<Value, ExpectedError> {
        let params: Vec<Value> = params.parse()?;
        if let Err(err) = validation::l1_tx_log::verify(&params) {
            log::warn!("{}", err);
            return Err(ExpectedError::RequestError(String::from("request params must be array! params=[{\"block_number\": 123, \"queue_index\": 12}, {\"block_number\": 234, \"queue_index\": 23}]")));
        }
        let param_vec: Vec<(u64, u64)> = params.into_iter()
            .map(|v| {
                let o = v.as_object().unwrap();
                (o.get("block_number").unwrap().as_u64().unwrap(), o.get("queue_index").unwrap().as_u64().unwrap())
            })
            .collect::<Vec<(u64, u64)>>();
        for (block_number, queue_index) in param_vec.iter() {
            let _ = self_sender.send(L1TxLogMsg::new(*block_number, *queue_index))?;
        }
        let param_vec_str = param_vec.into_iter()
            .map(|(block_number, queue_index)| { format!("{{block_number: {}, queue_index: {}}}", block_number, queue_index) })
            .collect::<Vec<String>>();
        Ok(Value::String(format!("retry job registered! task={}, params=[{}]", TASK_NAME, param_vec_str.join(", "))))
    }
}