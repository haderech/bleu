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
use crate::libs::subscribe::load_task_from_json;
use crate::message;
use crate::plugin::postgres::{PostgresMsg, PostgresPlugin};
use crate::types::channel::MultiSender;
use crate::types::subscribe::SubscribeEvent;

#[appbase_plugin(PostgresPlugin)]
pub struct L1TxLogPlugin {
    sub_event: Option<SubscribeEvent>,
    senders: Option<MultiSender>,
    receiver: Option<Receiver>,
}

const CHAIN: &str = "ethereum";
const TASK_PREFIX: &str = "task:ethereum";
const TASK_NAME: &str = "l1_tx_log";
const TASK_FILE: &str = "task/l1_tx_log.json";
const ABI_FILE: &str = "abi/tx_enqueued.json";
const EVENT: &str = "TransactionEnqueued";
const TOPIC0: &str = "0x4b388aecf9fa6cc92253704e5975a6129a4f735bdbd99567df4ed0094ee4ceb5";

type LogKeyValue = HashMap<String, String>;

message!(L1TxLogMsg; {block_number: u64}, {queue_index: u64});

impl Plugin for L1TxLogPlugin {
    fn new() -> Self {
        L1TxLogPlugin {
            sub_event: None,
            senders: None,
            receiver: None,
        }
    }

    fn init(&mut self) {
        let senders = MultiSender::new(vec!("postgres" /*"elasticsearch"*/));
        self.senders = Some(senders.to_owned());
        self.receiver = Some(APP.channels.subscribe(TASK_NAME));
        self.sub_event = Some(load_task_from_json(TASK_FILE, CHAIN, TASK_PREFIX, TASK_NAME).expect(format!("failed to load task! task={}", TASK_NAME).as_str()));
    }

    fn startup(&mut self) {
        let receiver = self.receiver.take().unwrap();
        let senders = self.senders.take().unwrap();
        let sub_event = self.sub_event.take().unwrap();
        let app = APP.quit_handle().unwrap();

        Self::recv(receiver, sub_event, senders, app);
    }

    fn shutdown(&mut self) {}
}

impl L1TxLogPlugin {
    fn recv(mut receiver: Receiver, sub_event: SubscribeEvent, senders: MultiSender, app: QuitHandle) {
        APP.spawn_blocking(move || {
            if let Ok(message) = receiver.try_recv() {
                if let Err(err) = Self::message_handler(message.clone(), &sub_event, &senders) {
                    log::error!("{}, message={:?}", err.to_string(), message);
                }
            }
            if !app.is_quitting() {
                Self::recv(receiver, sub_event, senders, app);
            }
        });
    }

    fn message_handler(message: Value, sub_event: &SubscribeEvent, senders: &MultiSender) -> Result<(), ExpectedError> {
        let parsed_msg = message.as_object().unwrap();
        let block_number = get_u64(parsed_msg, "block_number")?;
        let block_number_hex = format!("0x{:x}", block_number);
        let queue_index = get_u64(parsed_msg, "queue_index")?;
        let queue_index_hex = format!("{:x}", queue_index);
        let req_url = sub_event.active_node();
        let req_body = json!({
                    "jsonrpc": "2.0",
                    "method": "eth_getLogs",
                    "params": [{
                        "fromBlock": block_number_hex,
                        "toBlock": block_number_hex,
                        "topic": [ TOPIC0 ]
                    }],
                    "id": 1
                });
        let response = request::post(req_url.as_str(), req_body.to_string().as_str())?;
        if !libs::subscribe::is_log_created(&response, "result") {
            return Err(ExpectedError::NoneError(format!("logs does not created! block_number={}, topic={}", block_number, TOPIC0)));
        }
        let logs = get_array(&response, "result")?;
        let pg_sender = senders.get("postgres");
        for log_value in logs {
            if Self::is_matched_log(ABI_FILE, EVENT, log_value, queue_index_hex.clone())? {
                let mut converted_log = hex_to_decimal_converter(opt_to_result(log_value.as_object())?, vec!["blockNumber", "logIndex", "transactionIndex"])?;
                converted_log.insert(String::from("queue_index"), Value::String(queue_index.to_string()));
                let _ = pg_sender.send(PostgresMsg::new(String::from("ethereum_tx_logs"), Value::Object(converted_log)));
                break;
            }
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
}