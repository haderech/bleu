use appbase::channel;
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};

use crate::enumeration;
use crate::libs::serde::{get_str, get_string, get_string_vec, get_u64};
use crate::plugin::rocks::{RocksMethod, RocksMsg};
use crate::types::enumeration::Enumeration;
use crate::types::subscribe::SubscribeStatus::Working;

#[derive(Debug, Clone)]
pub struct SubscribeEvent {
    pub task_id: String,
    pub chain: String,
    pub task: String,
    pub start_idx: u64,
    pub curr_idx: u64,
    pub end_points: Vec<String>,
    pub end_point_idx: u16,
    pub filter: String,
    pub status: SubscribeStatus,
}

impl SubscribeEvent {
    // pub fn new(chain: &str, params: &Map<String, Value>) -> Self {
    //     let task = get_string(params, "task").unwrap();
    //     let start_idx = get_u64(params, "start_idx").unwrap();
    //     let filter_result = get_string(params, "filter");
    //     let filter = match filter_result {
    //         Ok(filter) => filter,
    //         Err(_) => String::from("")
    //     };
    //     SubscribeEvent {
    //         task_id: format!("task:{}:{}", chain, task),
    //         chain: String::from(chain),
    //         task,
    //         start_idx,
    //         curr_idx: start_idx,
    //         end_points: get_string_vec(params, "end_points"),
    //         end_point_idx: 0,
    //         filter,
    //         status: SubscribeStatus::Working,
    //     }
    // }

    pub fn from(params: &Map<String, Value>) -> Self {
        SubscribeEvent {
            task_id: get_string(params, "task_id").unwrap(),
            chain: get_string(params, "chain").unwrap(),
            task: get_string(params, "task").unwrap(),
            start_idx: get_u64(params, "start_idx").unwrap(),
            curr_idx: get_u64(params, "curr_idx").unwrap(),
            end_points: get_string_vec(params, "end_points"),
            end_point_idx: get_u64(params, "end_point_idx").unwrap() as u16,
            filter: get_string(params, "filter").unwrap(),
            status: SubscribeStatus::find(get_str(params, "status").unwrap()).unwrap(),
        }
    }

    pub fn load(task_id: String, task: String, chain: String, task_map: &Map<String, Value>) -> Self {
        let start_idx = get_u64(task_map, "start_idx").unwrap();
        let end_points = get_string_vec(task_map, "end_points");
        let filter = get_string(task_map, "filter").unwrap();
        SubscribeEvent {
            task_id,
            chain,
            task,
            start_idx,
            curr_idx: start_idx,
            end_points,
            end_point_idx: 0,
            filter,
            status: SubscribeStatus::Working,
        }
    }

    pub fn is_workable(&self) -> bool {
        vec!(Working).contains(&self.status)
    }

    pub fn handle_error(&mut self, rocks_channel: &channel::Sender, err_msg: String) {
        log::error!("{}", err_msg.clone());

        if usize::from(self.end_point_idx) + 1 < self.end_points.len() {
            self.end_point_idx += 1;
        } else {
            self.status = SubscribeStatus::Error;
        }
        let task = SubscribeTask::from(self, err_msg.clone());
        let msg = RocksMsg::new(RocksMethod::Put, self.task_id.clone(), Value::String(json!(task).to_string()));
        let _ = rocks_channel.send(msg);
    }

    pub fn active_node(&self) -> String {
        let usize_idx = usize::from(self.end_point_idx);
        self.end_points[usize_idx].clone()
    }

    pub fn next_idx(&mut self) { self.curr_idx += 1; }

    pub fn get_task_id(&self) -> String { self.task_id.clone() }

    pub fn status(&mut self, status: SubscribeStatus) { self.status = status; }

    pub fn get_filter(&self) -> String { self.filter.clone() }
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub struct SubscribeTask {
    pub task_id: String,
    pub chain: String,
    pub task: String,
    pub start_idx: u64,
    pub curr_idx: u64,
    pub end_points: Vec<String>,
    pub end_point_idx: u16,
    pub filter: String,
    pub status: String,
    pub err_msg: String,
}

impl SubscribeTask {
    pub fn from(sub_event: &SubscribeEvent, err_msg: String) -> Self {
        SubscribeTask {
            task_id: sub_event.task_id.clone(),
            chain: sub_event.chain.clone(),
            task: sub_event.task.clone(),
            start_idx: sub_event.start_idx,
            curr_idx: sub_event.curr_idx,
            end_points: sub_event.end_points.clone(),
            end_point_idx: sub_event.end_point_idx,
            filter: sub_event.filter.clone(),
            status: sub_event.status.value(),
            err_msg,
        }
    }

    pub fn get_task_id(&self) -> String {
        self.task_id.clone()
    }
}

pub trait RetryJob {
    fn get_retry_id(&self) -> String;
    fn get_retry_count(&self) -> u32;
    fn decrease_retry_count(&mut self);
    fn is_retry_available(&self) -> bool;
}

enumeration!(SubscribeStatus; {Working: "working"}, {Stopped: "stopped"}, {Error: "error"});
enumeration!(TaskMethod; {Start: "start"}, {Stop: "stop"});

#[cfg(test)]
mod subscribe_test {
    use serde_json::{json, Map, Value};

    use crate::types::subscribe::SubscribeEvent;

    #[test]
    fn subscribe_event_task_id_test() {
        let mut params = Map::new();
        params.insert(String::from("task"), json!("cosmoshub-4"));
        params.insert(String::from("start_idx"), json!(1u64));
        params.insert(String::from("end_points"), json!(["https://api.cosmos.network"]));
        params.insert(String::from("filter"), Value::String(String::from("")));

        let subscribe_event = SubscribeEvent::new("tendermint", &params);
        assert_eq!(subscribe_event.task_id, "task:tendermint:cosmoshub-4");
    }

    #[test]
    fn subscribe_event_is_workable_test() {
        let mut params = Map::new();
        params.insert(String::from("task"), json!("cosmoshub-4"));
        params.insert(String::from("start_idx"), json!(1u64));
        params.insert(String::from("end_points"), json!(["https://api.cosmos.network"]));
        params.insert(String::from("filter"), Value::String(String::from("")));

        let subscribe_event = SubscribeEvent::new("tendermint", &params);
        assert!(subscribe_event.is_workable());
    }
}
