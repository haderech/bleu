use std::collections::HashMap;
use std::fs;

use serde_json::Value;

use crate::error::error::ExpectedError;
use crate::libs::opts::opt_to_result;
use crate::libs::rocks::get_by_prefix_static;
use crate::plugin::rocks::RocksDB;
use crate::types::subscribe::SubscribeEvent;

pub fn load_task(rocksdb: RocksDB, file_path: &str, chain: &str, prefix: &str) -> HashMap<String, SubscribeEvent> {
    let mut sub_events: HashMap<String, SubscribeEvent> = HashMap::new();
    match load_task_from_json(file_path, chain, prefix) {
        Ok(task_from_json) => {
            for (task_id, sub_event) in task_from_json {
                sub_events.insert(task_id, sub_event);
            }
        }
        Err(err) => println!("{}", err.to_string())
    };
    match load_task_from_rocksdb(rocksdb, prefix) {
        Ok(task_from_rocksdb) => {
            for (task_id, sub_event) in task_from_rocksdb {
                sub_events.insert(task_id, sub_event);
            }
        }
        Err(err) => println!("{}", err.to_string())
    };
    sub_events
}

fn load_task_from_rocksdb(rocksdb: RocksDB, prefix: &str) -> Result<HashMap<String, SubscribeEvent>, ExpectedError> {
    let tasks_value = get_by_prefix_static(&rocksdb, prefix);
    let tasks_array = tasks_value.as_array().unwrap();

    let mut sub_events: HashMap<String, SubscribeEvent> = HashMap::new();
    for task_value in tasks_array.iter() {
        let task_map = task_value.as_object().unwrap();
        let sub_event = SubscribeEvent::from(task_map);
        sub_events.insert(sub_event.get_task_id(), sub_event);
    }
    Ok(sub_events)
}

fn load_task_from_json(file_path: &str, chain: &str, prefix: &str) -> Result<HashMap<String, SubscribeEvent>, ExpectedError> {
    let json_str = fs::read_to_string(file_path)?;
    let json_value: Value = serde_json::from_str(json_str.as_str())?;
    let schema_map = opt_to_result(json_value.as_object())?;

    let mut sub_events: HashMap<String, SubscribeEvent> = HashMap::new();
    for (sub_id, task_value) in schema_map.iter() {
        let task_map = opt_to_result(task_value.as_object())?;
        let task_id = format!("{}:{}", prefix, sub_id.clone());
        let sub_event = SubscribeEvent::load(task_id.clone(), sub_id.to_owned(), String::from(chain), task_map);
        sub_events.insert(task_id, sub_event);
    }
    Ok(sub_events)
}

#[cfg(test)]
mod subscribe {
    use crate::libs::subscribe::load_task_from_json;

    #[test]
    fn load_task_test() {
        let sub_events = load_task_from_json("task/optimism.json", "optimism", "task:optimism");
        for (task_id, sub_event) in sub_events.iter() {
            println!("task_id={}, sub_event={:?}", task_id, sub_event);
        }
        assert_eq!(sub_events.len(), 4);
    }
}