use std::collections::HashMap;
use std::sync::Arc;

use appbase::prelude::*;
use futures::lock::Mutex as FutureMutex;
use jsonrpc_core::Params;
use jsonrpc_core::serde_json::Map;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::enumeration;
use crate::error::error::ExpectedError;
use crate::libs::request;
use crate::libs::rocks::{get_by_prefix_static, get_static};
use crate::libs::serde::{get_array, get_object, get_str};
use crate::libs::subscribe::load_task;
use crate::message;
use crate::plugin::jsonrpc::JsonRpcPlugin;
use crate::plugin::postgres::{PostgresMsg, PostgresPlugin};
use crate::plugin::rocks::{RocksMethod, RocksMsg, RocksPlugin};
use crate::types::channel::MultiSender;
use crate::types::enumeration::Enumeration;
use crate::types::subscribe::{SubscribeEvent, SubscribeStatus, SubscribeTask};
use crate::validation::{start_subscribe, stop_subscribe};

#[appbase_plugin(JsonRpcPlugin, RocksPlugin, PostgresPlugin)]
pub struct OptimismPlugin {
    sub_events: Option<SubscribeEvents>,
    senders: Option<MultiSender>,
    receiver: Option<Receiver>,
}

const CHAIN: &str = "optimism";
const TASK_PREFIX: &str = "task:optimism";
const TASK_FILE: &str = "task/optimism.json";

type SubscribeEvents = Arc<FutureMutex<HashMap<String, SubscribeEvent>>>;

message!((OptimismMsg; {value: Value}); (OptimismMethod; {Start: "start"}, {Stop: "stop"}));

impl Plugin for OptimismPlugin {
    fn new() -> Self {
        OptimismPlugin {
            sub_events: None,
            senders: None,
            receiver: None,
        }
    }

    fn init(&mut self) {
        let rocksdb = APP.run_with::<RocksPlugin, _, _>(|rocks| rocks.get_db());
        self.sub_events = Some(Arc::new(FutureMutex::new(load_task(rocksdb, TASK_FILE, CHAIN, TASK_PREFIX))));
        let senders = MultiSender::new(vec!("optimism", "rocks", "postgres", /*"elasticsearch"*/));
        self.senders = Some(senders.to_owned());
        self.receiver = Some(APP.channels.subscribe("optimism"));

        self.register_jsonrpc();
    }

    fn startup(&mut self) {
        let monitor = self.receiver.take().unwrap();
        let sub_events = self.sub_events.take().unwrap();
        let senders = self.senders.take().unwrap();
        let app = APP.quit_handle().unwrap();

        Self::recv(monitor, sub_events, senders, app);
    }

    fn shutdown(&mut self) {}
}

impl OptimismPlugin {
    fn recv(mut monitor: Receiver, sub_events: SubscribeEvents, senders: MultiSender, app: QuitHandle) {
        APP.spawn_blocking(move || {
            if let Some(mut locked_events) = sub_events.try_lock() {
                if let Ok(message) = monitor.try_recv() {
                    Self::message_handler(message, &mut locked_events, &senders);
                }
                for (_, sub_event) in locked_events.iter_mut() {
                    if sub_event.is_workable() {
                        match Self::event_handler(sub_event, &senders) {
                            Ok(_) => Self::task_syncer(sub_event, &senders),
                            Err(err) => Self::error_handler(err, sub_event, &senders)
                        }
                    }
                }
            }
            if !app.is_quitting() {
                Self::recv(monitor, sub_events, senders, app);
            }
        });
    }

    fn is_value_created(res_body: &Map<String, Value>, value_name: &str) -> bool {
        let value = res_body.get(value_name);
        value.is_some() && !value.unwrap().is_null()
    }

    fn message_handler(message: Value, sub_events: &mut HashMap<String, SubscribeEvent>, senders: &MultiSender) {
        let parsed_msg = message.as_object().unwrap();
        let method = OptimismMethod::find(get_str(parsed_msg, "method").unwrap()).unwrap();
        let params = get_object(parsed_msg, "value").unwrap();
        let task_id = get_str(&params, "task_id").unwrap();
        let sub_event = sub_events.get_mut(task_id).unwrap();
        match method {
            OptimismMethod::Start => sub_event.status(SubscribeStatus::Working),
            OptimismMethod::Stop => sub_event.status(SubscribeStatus::Stopped)
        };
        let sub_task = SubscribeTask::from(sub_event, String::from(""));
        let rocks_sender = senders.get("rocks");
        let _ = rocks_sender.send(RocksMsg::new(RocksMethod::Put, sub_event.get_task_id(), Value::String(json!(sub_task).to_string())));
    }

    fn event_handler(sub_event: &mut SubscribeEvent, senders: &MultiSender) -> Result<(), ExpectedError> {
        let pg_sender = senders.get("postgres");
        let _ = match sub_event.get_sub_id() {
            "tx_json_rpc" => Self::tx_json_rpc_handler(sub_event, pg_sender)?,
            "tx_batch_dtl" => Self::tx_batch_dtl_handler(sub_event, pg_sender)?,
            "enqueue_dtl" => Self::enqueue_dtl_handler(sub_event, pg_sender)?,
            "state_batch_dtl" => Self::state_batch_dtl_handler(sub_event, pg_sender)?,
            _ => return Err(ExpectedError::NoneError(format!("unsupported subscription id! sub_id={}", sub_event.get_sub_id())))
        };
        Ok(())
    }

    fn error_handler(err: ExpectedError, sub_event: &mut SubscribeEvent, senders: &MultiSender) {
        let rocks_sender = senders.get("rocks");
        sub_event.handle_error(&rocks_sender, err.to_string());
    }

    fn create_req_url(node_url: String, curr_idx: u64) -> String {
        let adjusted_url = adjust_url(node_url);
        format!("{adjusted_url}{curr_idx}", adjusted_url = adjusted_url, curr_idx = curr_idx)
    }

    fn tx_json_rpc_handler(sub_event: &mut SubscribeEvent, pg_sender: Sender) -> Result<(), ExpectedError> {
        let req_url = sub_event.active_node();
        let hex_idx = format!("0x{:x}", sub_event.curr_idx);
        let req_body = json!({
                    "jsonrpc": "2.0",
                    "method": "eth_getBlockByNumber",
                    "params": [ hex_idx, true ],
                    "id": 1
                });
        let response = request::post(req_url.as_str(), req_body.to_string().as_str())?;
        match Self::is_value_created(&response, "result") {
            true => {
                let block = get_object(&response, "result")?;
                let _ = pg_sender.send(PostgresMsg::new(String::from("optimism_blocks"), Value::Object(block.clone())))?;
                let txs = get_array(&block, "transactions")?;
                for tx in txs.iter() {
                    let _ = pg_sender.send(PostgresMsg::new(String::from("optimism_block_txs"), tx.clone()))?;
                }
                sub_event.next_idx();
            }
            false => println!("{}", format!("waiting for tx created...sub_id={}", sub_event.sub_id))
        };
        Ok(())
    }

    fn tx_batch_dtl_handler(sub_event: &mut SubscribeEvent, pg_sender: Sender) -> Result<(), ExpectedError> {
        let req_url = Self::create_req_url(sub_event.active_node(), sub_event.curr_idx);
        let response = request::get(req_url.as_str())?;
        match Self::is_value_created(&response, "batch") {
            true => {
                let batch = get_object(&response, "batch")?;
                let _ = pg_sender.send(PostgresMsg::new(String::from("optimism_tx_batches"), Value::Object(batch.clone())))?;
                let txs = get_array(&response, "transactions")?;
                for tx in txs.iter() {
                    let _ = pg_sender.send(PostgresMsg::new(String::from("optimism_txs"), tx.clone()))?;
                }
                sub_event.next_idx();
            }
            false => println!("{}", format!("waiting for tx batch created...sub_id={}", sub_event.sub_id))
        };
        Ok(())
    }

    fn enqueue_dtl_handler(sub_event: &mut SubscribeEvent, pg_sender: Sender) -> Result<(), ExpectedError> {
        Ok(())
    }

    fn state_batch_dtl_handler(sub_event: &mut SubscribeEvent, pg_sender: Sender) -> Result<(), ExpectedError> {
        let req_url = Self::create_req_url(sub_event.active_node(), sub_event.curr_idx);
        let response = request::get(req_url.as_str())?;
        match Self::is_value_created(&response, "batch") {
            true => {
                let batch = get_object(&response, "batch")?;
                let _ = pg_sender.send(PostgresMsg::new(String::from("optimism_state_batches"), Value::Object(batch.clone())))?;
                let txs = get_array(&response, "stateRoots")?;
                for tx in txs.iter() {
                    let _ = pg_sender.send(PostgresMsg::new(String::from("optimism_state_roots"), tx.clone()))?;
                }
                sub_event.next_idx();
            }
            false => println!("{}", format!("waiting for state root batch created...sub_id={}", sub_event.sub_id))
        };
        Ok(())
    }

    fn register_jsonrpc(&self) {
        let optimism_sender = self.senders.as_ref().unwrap().get("optimism");
        let rocks_db = APP.run_with::<RocksPlugin, _, _>(|rocks| rocks.get_db());
        APP.run_with::<JsonRpcPlugin, _, _>(|jsonrpc| {
            jsonrpc.add_method(String::from("optimism_task_start"), move |params: Params| {
                let params: Map<String, Value> = params.parse().unwrap();
                let response = match start_subscribe::verify(&params) {
                    Ok(_) => {
                        let task_id = get_str(&params, "task_id").unwrap();
                        let task = get_static(&rocks_db, task_id);
                        match task.is_null() {
                            true => json!({"error": format!("task does not exist! task_id={}", task_id)}),
                            false => {
                                let _ = optimism_sender.send(OptimismMsg::new(OptimismMethod::Start, Value::Object(params.to_owned())));
                                Value::String(format!("start task requested! task_id={}", task_id))
                            }
                        }
                    }
                    Err(err) => json!({"error": err.to_string()})
                };
                Box::new(futures::future::ok(response))
            });
        });

        let optimism_sender = self.senders.as_ref().unwrap().get("optimism");
        let rocks_db = APP.run_with::<RocksPlugin, _, _>(|rocks| rocks.get_db());
        APP.run_with::<JsonRpcPlugin, _, _>(|jsonrpc| {
            jsonrpc.add_method(String::from("optimism_task_stop"), move |params: Params| {
                let params: Map<String, Value> = params.parse().unwrap();
                let response = match stop_subscribe::verify(&params) {
                    Ok(_) => {
                        let task_id = get_str(&params, "task_id").unwrap();
                        let task = get_static(&rocks_db, task_id);
                        match task.is_null() {
                            true => json!({"error": format!("task does not exist! task_id={}", task_id)}),
                            false => {
                                let _ = optimism_sender.send(OptimismMsg::new(OptimismMethod::Stop, Value::Object(params.to_owned())));
                                Value::String(format!("stop task requested! task_id={}", task_id))
                            }
                        }
                    }
                    Err(err) => json!({"error": err.to_string()})
                };
                Box::new(futures::future::ok(response))
            });
        });

        let rocks_db = APP.run_with::<RocksPlugin, _, _>(|rocks| rocks.get_db());
        APP.run_with::<JsonRpcPlugin, _, _>(|jsonrpc| {
            jsonrpc.add_method(String::from("optimism_tasks"), move |_| {
                let tasks = get_by_prefix_static(&rocks_db, TASK_PREFIX);
                Box::new(futures::future::ok(tasks))
            });
        });
    }

    fn task_syncer(sub_event: &mut SubscribeEvent, senders: &MultiSender) {
        let task = SubscribeTask::from(&sub_event, String::from(""));
        let rocks_sender = senders.get("rocks");
        let _ = rocks_sender.send(RocksMsg::new(RocksMethod::Put, task.get_task_id(), Value::String(json!(task).to_string())));
    }
}

fn adjust_url(url: String) -> String {
    let mut tmp_url = url.clone();
    if !tmp_url.ends_with("/") {
        tmp_url += "/";
    }
    tmp_url.to_owned()
}

#[cfg(test)]
mod optimism {
    use serde_json::{Map, Value};

    use crate::plugin::optimism;

    #[test]
    fn adjust_url_test() {
        let example_url = optimism::adjust_url(String::from("https://example.com"));
        assert_eq!("https://example.com/", example_url);

        let example_url2 = optimism::adjust_url(String::from("https://example2.com/"));
        assert_eq!("https://example2.com/", example_url2);
    }

    #[test]
    fn get_req_url_test() {
        let example_url = optimism::adjust_url(String::from("https://example.com"));
        let curr_idx = 1u64;
        let req_url = optimism::OptimismPlugin::create_req_url(example_url, curr_idx);
        println!("{}", req_url);

        assert_eq!("https://example.com/1", req_url);
    }

    #[test]
    fn is_value_created_test() {
        let mut object = Map::new();
        object.insert(String::from("batch"), Value::Null);
        let is_value_created = optimism::OptimismPlugin::is_value_created(&object, "batch");
        assert!(!is_value_created);

        let mut object2 = Map::new();
        let mut sub_object = Map::new();
        sub_object.insert(String::from("test"), Value::String(String::from("test_value")));
        object2.insert(String::from("batch"), Value::Object(sub_object));
        let is_value_created = optimism::OptimismPlugin::is_value_created(&object2, "batch");
        assert!(is_value_created);
    }
}