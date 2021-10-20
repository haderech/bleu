use std::collections::HashMap;
use std::fs;
use std::sync::Arc;

use appbase::prelude::*;
use futures::lock::Mutex as FutureMutex;
use jsonrpc_core::serde_json::Map;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::enumeration;
use crate::error::error::ExpectedError;
use crate::libs::opts::opt_to_result;
use crate::libs::request;
use crate::libs::serde::get_array;
use crate::message;
use crate::plugin::postgres::{PostgresMsg, PostgresPlugin};
use crate::types::channel::MultiChannel;
use crate::types::enumeration::Enumeration;
use crate::types::subscribe::SubscribeEvent;

#[appbase_plugin(PostgresPlugin)]
pub struct OptimismPlugin {
    sub_events: Option<SubscribeEvents>,
    channels: Option<MultiChannel>,
    monitor: Option<Receiver>,
}

const CHAIN: &str = "optimism";
const TASK_PREFIX: &str = "task:optimism";
const TASK_FILE: &str = "task/optimism.json";

type SubscribeEvents = Arc<FutureMutex<HashMap<String, SubscribeEvent>>>;

message!((OptimismMsg; {value: Value}); (OptimismMethod; {Subscribe: "subscribe"}, {Resubscribe: "resubscribe"}, {Stop: "stop"}, {Unsubscribe: "unsubscribe"}));

impl Plugin for OptimismPlugin {
    fn new() -> Self {
        OptimismPlugin {
            sub_events: None,
            channels: None,
            monitor: None,
        }
    }

    fn init(&mut self) {
        self.sub_events = match Self::load_task() {
            Ok(load_task) => Some(Arc::new(FutureMutex::new(load_task))),
            Err(err) => {
                log::error!("{}", err.to_string());
                Some(Arc::new(FutureMutex::new(HashMap::new())))
            }
        };
        let channels = MultiChannel::new(vec!("optimism", "rocks", "postgres", /*"elasticsearch"*/));
        self.channels = Some(channels.to_owned());
        self.monitor = Some(APP.channels.subscribe("optimism"));
    }

    fn startup(&mut self) {
        let monitor = self.monitor.take().unwrap();
        let sub_events = self.sub_events.take().unwrap();
        let channels = self.channels.take().unwrap();
        let app = APP.quit_handle().unwrap();

        Self::recv(monitor, sub_events, channels, app);
    }

    fn shutdown(&mut self) {}
}

impl OptimismPlugin {
    fn recv(monitor: Receiver, sub_events: SubscribeEvents, channels: MultiChannel, app: QuitHandle) {
        APP.spawn_blocking(move || {
            if let Some(mut locked_events) = sub_events.try_lock() {
                for (_, sub_event) in locked_events.iter_mut() {
                    if let Err(err) = Self::event_handler(sub_event, &channels) {
                        Self::error_handler(err, sub_event, &channels);
                    }
                }
            }
            if !app.is_quitting() {
                Self::recv(monitor, sub_events, channels, app);
            }
        });
    }

    fn load_task() -> Result<HashMap<String, SubscribeEvent>, ExpectedError> {
        let json_str = fs::read_to_string(TASK_FILE)?;
        let json_value: Value = serde_json::from_str(json_str.as_str())?;
        let schema_map = opt_to_result(json_value.as_object())?;

        let mut sub_events: HashMap<String, SubscribeEvent> = HashMap::new();
        for (sub_id, task_value) in schema_map.iter() {
            let task_map = opt_to_result(task_value.as_object())?;

            let task_id = format!("{}:{}", TASK_PREFIX, sub_id.clone());
            let sub_event = SubscribeEvent::load(task_id.clone(), sub_id.to_owned(), String::from(CHAIN), task_map);
            sub_events.insert(task_id, sub_event);
        }
        Ok(sub_events.to_owned())
    }

    fn is_batch_created(res_body: &Map<String, Value>) -> bool {
        let batch = res_body.get("batch");
        batch.is_some() && !batch.unwrap().is_null()
    }

    fn save_postgres(pg_sender: Sender, data: Map<String, Value>) -> Result<(), ExpectedError> {
        let _ = pg_sender.send(PostgresMsg::new(String::from("optimism_batches"), Value::Object(data.clone())))?;
        let txs = get_array(&data, "transactions")?;
        for tx in txs.iter() {
            let _ = pg_sender.send(PostgresMsg::new(String::from("optimism_txs"), tx.clone()))?;
        }
        Ok(())
    }

    fn event_handler(sub_event: &mut SubscribeEvent, channels: &MultiChannel) -> Result<(), ExpectedError> {
        let req_url = get_req_url(sub_event.active_node(), sub_event.curr_idx);
        let response = request::get(req_url.as_str())?;
        if let true = Self::is_batch_created(&response) {
            sub_event.next_idx();
            let pg_sender = channels.get("postgres");
            let _ = Self::save_postgres(pg_sender, response)?;
        } else {
            println!("waiting for batch created...");
        }
        Ok(())
    }

    fn error_handler(err: ExpectedError, sub_event: &mut SubscribeEvent, channels: &MultiChannel) {
        let rocks_sender = channels.get("rocks");
        sub_event.handle_error(&rocks_sender, err.to_string());
    }
}

fn get_req_url(node_url: String, curr_idx: u64) -> String {
    let adjusted_url = adjust_url(node_url);
    format!("{adjusted_url}{curr_idx}", adjusted_url = adjusted_url, curr_idx = curr_idx)
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
    use crate::plugin::optimism::OptimismPlugin;

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
        let req_url = optimism::get_req_url(example_url, curr_idx);
        println!("{}", req_url);

        assert_eq!("https://example.com/1", req_url);
    }

    #[test]
    fn is_batch_created_test() {
        let mut object = Map::new();
        object.insert(String::from("batch"), Value::Null);
        let is_batch_created = optimism::OptimismPlugin::is_batch_created(&object);
        assert!(!is_batch_created);

        let mut object2 = Map::new();
        let mut sub_object = Map::new();
        sub_object.insert(String::from("test"), Value::String(String::from("test_value")));
        object2.insert(String::from("batch"), Value::Object(sub_object));
        let is_batch_created = optimism::OptimismPlugin::is_batch_created(&object2);
        assert!(is_batch_created);
    }

    #[test]
    fn load_task_test() {
        let sub_events = OptimismPlugin::load_task().unwrap();
        for (task_id, sub_event) in sub_events.iter() {
            println!("task_id={}, sub_event={:?}", task_id, sub_event);
        }
        assert_eq!(sub_events.len(), 4);
    }
}