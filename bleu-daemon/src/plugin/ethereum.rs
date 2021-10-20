use std::collections::HashMap;
use std::sync::Arc;

use appbase::prelude::*;
use futures::lock::Mutex as FutureMutex;
use jsonrpc_core::Params;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

use crate::{enumeration, libs, message};
use crate::error::error::ExpectedError;
use crate::libs::opts::{opt_ref_to_result, opt_to_result};
use crate::libs::request;
use crate::libs::rocks::{get_by_prefix_static, get_static};
use crate::libs::serde::{get_array, get_object, get_str, get_string};
// use crate::plugin::elasticsearch::{ElasticsearchMsg, ElasticsearchPlugin};
use crate::plugin::jsonrpc::JsonRpcPlugin;
use crate::plugin::postgres::{PostgresMsg, PostgresPlugin};
use crate::plugin::rocks::{RocksMethod, RocksMsg, RocksPlugin};
use crate::types::channel::MultiChannel;
use crate::types::enumeration::Enumeration;
use crate::types::subscribe::{SubscribeEvent, SubscribeStatus, SubscribeTask};
use crate::validation::{get_task, resubscribe, stop_subscribe, subscribe, unsubscribe};

// #[appbase_plugin(JsonRpcPlugin, RocksPlugin, PostgresPlugin, ElasticsearchPlugin)]
#[appbase_plugin(JsonRpcPlugin, RocksPlugin, PostgresPlugin)]
pub struct EthereumPlugin {
    sub_events: Option<SubscribeEvents>,
    channels: Option<MultiChannel>,
    monitor: Option<Receiver>,
}

const CHAIN: &str = "ethereum";
const TASK_PREFIX: &str = "task:ethereum";

type SubscribeEvents = Arc<FutureMutex<HashMap<String, SubscribeEvent>>>;

message!((EthereumMsg; {value: Value}); (EthereumMethod; {Subscribe: "subscribe"}, {Resubscribe: "resubscribe"}, {Stop: "stop"}, {Unsubscribe: "unsubscribe"}));

impl Plugin for EthereumPlugin {
    fn new() -> Self {
        EthereumPlugin {
            sub_events: None,
            channels: None,
            monitor: None,
        }
    }

    fn init(&mut self) {
        self.pre_init();
        self.register_jsonrpc();
        self.load_tasks();
    }

    fn startup(&mut self) {
        let mut monitor = self.monitor.take().unwrap();
        let sub_events = Arc::clone(self.sub_events.as_ref().unwrap());

        let mut rocks_channel = self.channels.as_ref().unwrap().get("rocks");
        let postgres_channel = self.channels.as_ref().unwrap().get("postgres");
        // let elasticsearch_channel = self.channels.as_ref().unwrap().get("elasticsearch");
        let app = APP.quit_handle().unwrap();

        APP.spawn_blocking(move || {
            loop {
                if app.is_quitting() {
                    break;
                }
                let sub_events_try_lock = sub_events.try_lock();
                if sub_events_try_lock.is_none() {
                    continue;
                }
                let mut sub_events_lock = sub_events_try_lock.unwrap();
                if let Ok(msg) = monitor.try_recv() {
                    Self::message_handler(&msg, &mut sub_events_lock, &mut rocks_channel);
                }

                for (_, sub_event) in sub_events_lock.iter_mut() {
                    if sub_event.is_workable() {
                        let block_result = Self::poll_block(sub_event);
                        match block_result {
                            Ok(block_value) => {
                                log::info!("event_id={}, block={}", sub_event.event_id(), block_value.to_string());

                                if let Err(error) = Self::save_postgres(block_value.clone(), &postgres_channel) {
                                    log::error!("{}", error.to_string());
                                };
                                // if let Err(error) = Self::save_elasticsearch(block_value.clone(), &elasticsearch_channel) {
                                //     log::error!("{}", error.to_string());
                                // }

                                Self::sync_event(&rocks_channel, sub_event);
                                sub_event.next_idx();
                            }
                            Err(err) => Self::error_handler(&rocks_channel, sub_event, err)
                        }
                    }
                }
            }
        });
    }

    fn shutdown(&mut self) {}
}

impl EthereumPlugin {
    fn pre_init(&mut self) {
        self.sub_events = Some(Arc::new(FutureMutex::new(HashMap::new())));
        let channels = MultiChannel::new(vec!("ethereum", "rocks", "postgres", /*"elasticsearch"*/));
        self.channels = Some(channels.to_owned());
        self.monitor = Some(APP.channels.subscribe("ethereum"));
    }

    fn register_jsonrpc(&self) {
        let eth_channel = self.channels.as_ref().unwrap().get("ethereum");
        let rocks_db = APP.run_with::<RocksPlugin, _, _>(|rocks| {
            rocks.get_db()
        });
        APP.run_with::<JsonRpcPlugin, _, _>(|jsonrpc| {
            jsonrpc.add_method(String::from("eth_subscribe"), move |params: Params| {
                let params: Map<String, Value> = params.parse().unwrap();
                let verified = subscribe::verify(&params);
                if verified.is_err() {
                    let mut error = Map::new();
                    error.insert(String::from("error"), Value::String(verified.unwrap_err().to_string()));
                    return Box::new(futures::future::ok(Value::Object(error)));
                }
                let task_id = SubscribeTask::task_id(CHAIN, &params);
                let value = get_static(&rocks_db, task_id.as_str());
                if value.is_null() {
                    let message = EthereumMsg::new(EthereumMethod::Subscribe, Value::Object(params.clone()));
                    let _ = eth_channel.send(message);

                    Box::new(futures::future::ok(Value::String(format!("subscription requested! task_id={}", task_id))))
                } else {
                    let mut error = Map::new();
                    error.insert(String::from("error"), Value::String(format!("already exist task! task_id={}", task_id)));
                    Box::new(futures::future::ok(Value::Object(error)))
                }
            });
        });

        let eth_channel = self.channels.as_ref().unwrap().get("ethereum");
        let rocks_db = APP.run_with::<RocksPlugin, _, _>(|rocks| {
            rocks.get_db()
        });
        APP.run_with::<JsonRpcPlugin, _, _>(|jsonrpc| {
            jsonrpc.add_method(String::from("eth_unsubscribe"), move |params: Params| {
                let params: Map<String, Value> = params.parse().unwrap();
                let verified = unsubscribe::verify(&params);
                if verified.is_err() {
                    let mut error = Map::new();
                    error.insert(String::from("error"), Value::String(verified.unwrap_err().to_string()));
                    return Box::new(futures::future::ok(Value::Object(error)));
                }

                let task_id = get_str(&params, "task_id").unwrap();
                let value = get_static(&rocks_db, task_id);
                if value.is_null() {
                    let mut error = Map::new();
                    error.insert(String::from("error"), Value::String(format!("task does not exist! task_id={}", task_id)));
                    Box::new(futures::future::ok(Value::Object(error)))
                } else {
                    let eth_msg = EthereumMsg::new(EthereumMethod::Unsubscribe, Value::Object(params.clone()));
                    let _ = eth_channel.send(eth_msg);

                    Box::new(futures::future::ok(Value::String(format!("unsubscription requested! task_id={}", task_id))))
                }
            });
        });

        let eth_channel = self.channels.as_ref().unwrap().get("ethereum");
        let rocks_db = APP.run_with::<RocksPlugin, _, _>(|rocks| {
            rocks.get_db()
        });
        APP.run_with::<JsonRpcPlugin, _, _>(|jsonrpc| {
            jsonrpc.add_method(String::from("eth_resubscribe"), move |params: Params| {
                let params: Map<String, Value> = params.parse().unwrap();
                let verified = resubscribe::verify(&params);
                if verified.is_err() {
                    let mut error = Map::new();
                    error.insert(String::from("error"), Value::String(verified.unwrap_err().to_string()));
                    return Box::new(futures::future::ok(Value::Object(error)));
                }

                let task_id = get_str(&params, "task_id").unwrap();
                let value = get_static(&rocks_db, task_id);
                if value.is_null() {
                    let mut error = Map::new();
                    error.insert(String::from("error"), Value::String(format!("subscription does not exist! task_id={}", task_id)));
                    Box::new(futures::future::ok(Value::Object(error)))
                } else {
                    let message = EthereumMsg::new(EthereumMethod::Resubscribe, Value::Object(params.clone()));
                    let _ = eth_channel.send(message);

                    Box::new(futures::future::ok(Value::String(format!("resubscription requested! task_id={}", task_id))))
                }
            });
        });

        let eth_channel = self.channels.as_ref().unwrap().get("ethereum");
        let rocks_db = APP.run_with::<RocksPlugin, _, _>(|rocks| {
            rocks.get_db()
        });
        APP.run_with::<JsonRpcPlugin, _, _>(|jsonrpc| {
            jsonrpc.add_method(String::from("eth_stop_subscription"), move |params: Params| {
                let params: Map<String, Value> = params.parse().unwrap();
                let verified = stop_subscribe::verify(&params);
                if verified.is_err() {
                    let mut error = Map::new();
                    error.insert(String::from("error"), Value::String(verified.unwrap_err().to_string()));
                    return Box::new(futures::future::ok(Value::Object(error)));
                }

                let task_id = get_str(&params, "task_id").unwrap();
                let value = get_static(&rocks_db, task_id);
                if value.is_null() {
                    let mut error = Map::new();
                    error.insert(String::from("error"), Value::String(format!("task does not exist! task_id={}", task_id)));
                    Box::new(futures::future::ok(Value::Object(error)))
                } else {
                    let eth_msg = EthereumMsg::new(EthereumMethod::Stop, Value::Object(params.clone()));
                    let _ = eth_channel.send(eth_msg);

                    Box::new(futures::future::ok(Value::String(format!("stop subscription requested! task_id={}", task_id))))
                }
            });
        });

        let rocks_db = APP.run_with::<RocksPlugin, _, _>(|rocks| {
            rocks.get_db()
        });
        APP.run_with::<JsonRpcPlugin, _, _>(|jsonrpc| {
            jsonrpc.add_method(String::from("eth_get_tasks"), move |params: Params| {
                let params: Map<String, Value> = params.parse().unwrap();
                let verified = get_task::verify(&params);
                if verified.is_err() {
                    let mut error = Map::new();
                    error.insert(String::from("error"), Value::String(verified.unwrap_err().to_string()));
                    return Box::new(futures::future::ok(Value::Object(error)));
                }

                let prefix = match params.get("task_id") {
                    None => TASK_PREFIX,
                    Some(task_id) => task_id.as_str().unwrap(),
                };
                let tasks = get_by_prefix_static(&rocks_db, prefix);
                Box::new(futures::future::ok(tasks))
            });
        });
    }

    fn load_tasks(&self) {
        let rocks_db = APP.run_with::<RocksPlugin, _, _>(|rocks| {
            rocks.get_db()
        });

        let raw_tasks = get_by_prefix_static(&rocks_db, TASK_PREFIX);
        let sub_events = Arc::clone(self.sub_events.as_ref().unwrap());
        raw_tasks.as_array().unwrap().iter()
            .for_each(|raw_task| {
                let task = raw_task.as_object().unwrap();
                let event = SubscribeEvent::from(task);
                let mut sub_events_lock = sub_events.try_lock().unwrap();
                sub_events_lock.insert(event.task_id.clone(), event);
            });
    }

    fn sync_event(rocks_channel: &Sender, sub_event: &mut SubscribeEvent) {
        let task = SubscribeTask::from(&sub_event, String::from(""));
        let task_id = task.task_id.clone();

        let msg = RocksMsg::new(RocksMethod::Put, task_id, Value::String(json!(task).to_string()));
        let _ = rocks_channel.send(msg);
    }

    fn error_handler(rocks_channel: &Sender, sub_event: &mut SubscribeEvent, error: ExpectedError) {
        match error {
            ExpectedError::BlockHeightError(err_msg) => log::info!("{}", err_msg),
            ExpectedError::FilterError(err_msg) => {
                log::info!("{}", err_msg);
                Self::sync_event(&rocks_channel, sub_event);
                sub_event.curr_idx += 1;
            }
            _ => sub_event.handle_error(&rocks_channel, error.to_string())
        };
    }

    fn message_handler(msg: &Value, sub_events: &mut HashMap<String, SubscribeEvent>, rocks_channel: &mut Sender) {
        let parsed_msg = msg.as_object().unwrap();
        let method = EthereumMethod::find(get_str(parsed_msg, "method").unwrap()).unwrap();
        let params = get_object(parsed_msg, "value").unwrap();
        match method {
            EthereumMethod::Subscribe => {
                let new_event = SubscribeEvent::new(CHAIN, &params);
                sub_events.insert(new_event.task_id.clone(), new_event.clone());

                let task = SubscribeTask::from(&new_event, String::from(""));
                let msg = RocksMsg::new(RocksMethod::Put, new_event.task_id, Value::String(json!(task).to_string()));
                let _ = rocks_channel.send(msg);
            }
            EthereumMethod::Unsubscribe => {
                let task_id = get_string(&params, "task_id").unwrap();
                sub_events.remove(&task_id);

                let msg = RocksMsg::new(RocksMethod::Delete, task_id, Value::Null);
                let _ = rocks_channel.send(msg);
            }
            EthereumMethod::Resubscribe => {
                let task_id = get_str(&params, "task_id").unwrap();
                let mut sub_event = sub_events.get(task_id).unwrap().clone();
                sub_event.end_point_idx = 0;
                sub_event.status = SubscribeStatus::Working;
                sub_events.insert(sub_event.task_id.clone(), sub_event.clone());

                let task = SubscribeTask::from(&sub_event, String::from(""));
                let msg = RocksMsg::new(RocksMethod::Put, sub_event.task_id, Value::String(json!(task).to_string()));
                let _ = rocks_channel.send(msg);
            }
            EthereumMethod::Stop => {
                let task_id = get_str(&params, "task_id").unwrap();
                let mut sub_event = sub_events.get(task_id).unwrap().clone();
                sub_event.status = SubscribeStatus::Stopped;
                sub_events.insert(sub_event.task_id.clone(), sub_event.clone());

                let task = SubscribeTask::from(&sub_event, String::from(""));
                let msg = RocksMsg::new(RocksMethod::Put, sub_event.task_id, Value::String(json!(task).to_string()));
                let _ = rocks_channel.send(msg);
            }
        };
    }

    fn poll_block(sub_event: &mut SubscribeEvent) -> Result<Value, ExpectedError> {
        let node_index = usize::from(sub_event.end_point_idx);
        let req_url = sub_event.end_points[node_index].clone();
        let hex_idx = format!("0x{:X}", sub_event.curr_idx);
        let req_body = json!({
            "jsonrpc": "2.0",
            "method": "eth_getBlockByNumber",
            "params": [ hex_idx, true ],
            "id": 1
        });
        let body = request::post(req_url.as_str(), req_body.to_string().as_str())?;
        let block = opt_to_result(body.get("result"))?;
        if block.is_null() {
            return Err(ExpectedError::BlockHeightError(String::from("block has not yet been created!")));
        }
        let block_object = opt_ref_to_result(block.as_object())?;
        let filter_result = libs::serde::filter(block_object, sub_event.filter.clone())?;
        if !filter_result {
            return Err(ExpectedError::FilterError(String::from("value does not match on filter condition!")));
        }
        Ok(block.clone())
    }

    fn save_postgres(block: Value, postgres: &Sender) -> Result<(), ExpectedError> {
        let mut block_object = opt_to_result(block.as_object())?.to_owned();
        block_object.insert(String::from("is_forked"), Value::Bool(false));
        let _ = postgres.send(PostgresMsg::new(String::from("eth_blocks"), Value::Object(block_object.clone())))?;

        let txs = get_array(&block_object, "transactions")?.to_owned();
        for tx in txs.into_iter() {
            let mut tx_object = opt_to_result(tx.as_object())?.to_owned();
            tx_object.insert(String::from("is_forked"), Value::Bool(false));
            let _ = postgres.send(PostgresMsg::new(String::from("eth_txs"), Value::Object(tx_object)))?;
        }
        Ok(())
    }

    // fn save_elasticsearch(block: Value, elasticsearch: &Sender) -> Result<(), ExpectedError> {
    //     let block_object = opt_to_result(block.as_object())?.to_owned();
    //     let txs = get_array(&block_object, "transactions")?.to_owned();
    //     let txs_size = txs.len();
    //
    //     let mut block_index = select_value(&block_object, vec!["hash", "number", "parentHash", "timestamp"])?;
    //     block_index.insert(String::from("txSize"), json!(txs_size));
    //     let block_index_id = get_str(&block_index, "hash")?;
    //     let _ = elasticsearch.send(ElasticsearchMsg::new(String::from("eth_blocks"), String::from(block_index_id), Value::Object(block_index)))?;
    //
    //     for tx in txs.into_iter() {
    //         let tx_object = opt_to_result(tx.as_object())?.to_owned();
    //         let tx_index = select_value(&tx_object, vec!["blockHash", "blockNumber", "from", "to", "hash"])?;
    //         let tx_index_id = get_str(&tx_index, "hash")?;
    //         let _ = elasticsearch.send(ElasticsearchMsg::new(String::from("eth_txs"), String::from(tx_index_id), Value::Object(tx_index)))?;
    //     }
    //     Ok(())
    // }
}

