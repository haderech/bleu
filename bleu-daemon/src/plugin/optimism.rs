use std::collections::HashMap;
use std::sync::Arc;

use appbase::prelude::*;
use futures::lock::Mutex as FutureMutex;
use jsonrpc_core::serde_json::Map;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::enumeration;
use crate::error::error::ExpectedError;
use crate::libs::request;
use crate::message;
use crate::plugin::postgres::PostgresPlugin;
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
        self.sub_events = Some(Arc::new(FutureMutex::new(HashMap::new())));
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
            if let Some(locked_events) = sub_events.try_lock() {
                for (_, mut sub_event) in locked_events.iter() {
                    let req_url = get_req_url(sub_event.active_node(), sub_event.curr_idx);
                    let res_result = request::get(req_url.as_str());
                    match res_result {
                        Ok(res_body) => {
                            print!("{:?}", res_body);
                            sub_event.next_idx();
                        },
                        Err(err) => eprintln!("{:?}", err)
                    }
                }
            }
            if !app.is_quitting() {
                Self::recv(monitor, sub_events, channels, app);
            }
        });
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
        let req_url = optimism::get_req_url(example_url, curr_idx);
        println!("{}", req_url);

        assert_eq!("https://example.com/1", req_url);
    }
}