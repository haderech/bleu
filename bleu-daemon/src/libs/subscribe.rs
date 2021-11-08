use std::collections::HashMap;
use std::fs;

use appbase::prelude::*;
use serde::de::DeserializeOwned;
use serde_json::{json, Map, Value};

use crate::error::error::ExpectedError;
use crate::libs::opt::opt_to_result;
use crate::libs::request::adjust_url;
use crate::libs::rocks::{get_by_prefix_static, get_static};
use crate::libs::serde::{filter, get_object, get_str};
use crate::plugin::rocks::{RocksDB, RocksMethod, RocksMsg};
use crate::types::channel::MultiSender;
use crate::types::enumeration::Enumeration;
use crate::types::subscribe::{RetryJob, SubscribeEvent, SubscribeStatus, SubscribeTask, TaskMethod};

pub fn task_loader(rocksdb: RocksDB, file_path: &str, chain: &str, task_prefix: &str, task_name: &str) -> Result<SubscribeEvent, ExpectedError> {
    match load_task_from_rocksdb(rocksdb, task_prefix, task_name) {
        Ok(sub_event) => Ok(sub_event),
        Err(_) => load_task_from_json(file_path, chain, task_prefix, task_name)
    }
}

fn load_task_from_rocksdb(rocksdb: RocksDB, task_prefix: &str, task: &str) -> Result<SubscribeEvent, ExpectedError> {
    let task_key = format!("{}:{}", task_prefix, task);
    let task_value = get_static(&rocksdb, &task_key)?;
    Ok(SubscribeEvent::from(opt_to_result(task_value.as_object())?))
}

pub fn load_task_from_json(file_path: &str, chain: &str, task_prefix: &str, task_name: &str) -> Result<SubscribeEvent, ExpectedError> {
    let json_str = fs::read_to_string(file_path)?;
    let json_value: Value = serde_json::from_str(json_str.as_str())?;
    let task_map = opt_to_result(json_value.as_object())?;
    let detail_map = get_object(task_map, task_name)?;
    let task_id = format!("{}:{}", task_prefix, task_name);
    Ok(SubscribeEvent::load(task_id, String::from(task_name), String::from(chain), detail_map))
}

pub fn is_value_created(res_body: &Map<String, Value>, value_name: &str) -> bool {
    let value = res_body.get(value_name);
    value.is_some() && !value.unwrap().is_null()
}

pub fn is_log_created(res_body: &Map<String, Value>, value_name: &str) -> bool {
    let value = res_body.get(value_name);
    value.is_some() && !value.unwrap().is_null() && !value.unwrap().as_array().unwrap().is_empty()
}

pub fn error_handler(err: ExpectedError, sub_event: &mut SubscribeEvent, senders: &MultiSender) {
    let rocks_sender = senders.get("rocks");
    match err {
        ExpectedError::BlockHeightError(err) => log::info!("{}", err.to_string()),
        ExpectedError::FilterError(err) => {
            log::info!("{}", err.to_string());
            task_syncer(sub_event, senders);
            sub_event.next_idx();
        }
        _ => sub_event.handle_error(&rocks_sender, err.to_string())
    };
}

pub fn task_syncer(sub_event: &SubscribeEvent, senders: &MultiSender) {
    let task = SubscribeTask::from(sub_event, String::from(""));
    let rocks_sender = senders.get("rocks");
    let _ = rocks_sender.send(RocksMsg::new(RocksMethod::Put, task.get_task_id(), Value::String(json!(task).to_string())));
}

pub fn create_req_url(node_url: String, curr_idx: u64) -> String {
    let adjusted_url = adjust_url(node_url);
    format!("{adjusted_url}{curr_idx}", adjusted_url = adjusted_url, curr_idx = curr_idx)
}

pub fn message_handler(message: Value, sub_event: &mut SubscribeEvent, senders: &MultiSender) {
    let parsed_msg = message.as_object().unwrap();
    let method = TaskMethod::find(get_str(parsed_msg, "method").unwrap()).unwrap();
    match method {
        TaskMethod::Start => sub_event.status(SubscribeStatus::Working),
        TaskMethod::Stop => sub_event.status(SubscribeStatus::Stopped)
    };
    let sub_task = SubscribeTask::from(sub_event, String::from(""));
    let rocks_sender = senders.get("rocks");
    let _ = rocks_sender.send(RocksMsg::new(RocksMethod::Put, sub_event.get_task_id(), Value::String(json!(sub_task).to_string())));
}

pub fn response_verifier(response: &Map<String, Value>, task_name: &str, value_name: &str, condition: String) -> Result<(), ExpectedError> {
    if !is_value_created(response, value_name) {
        return Err(ExpectedError::BlockHeightError(format!("waiting for data created...task={}", task_name)));
    }
    if !filter(response, condition)? {
        return Err(ExpectedError::FilterError(format!("not matched filter condition! task={}", task_name)));
    }
    Ok(())
}

pub fn load_retry_queue<T>(rocksdb: RocksDB, queue_prefix: &str) -> Result<HashMap<String, T>, ExpectedError>
    where
        T: DeserializeOwned + RetryJob {
    let values = get_by_prefix_static(&rocksdb, queue_prefix);
    let raw_item_vec = opt_to_result(values.as_array())?;
    let mut results = HashMap::new();
    for raw_item in raw_item_vec.into_iter() {
        let retry_job = serde_json::from_value::<T>(raw_item.to_owned())?;
        results.insert(retry_job.get_retry_id(), retry_job);
    }
    Ok(results)
}

pub fn save_retry_queue(rocks_sender: Sender, queue_key: String, queue_value: Value) -> Result<(), ExpectedError> {
    let _ = rocks_sender.send(RocksMsg::new(RocksMethod::Put, queue_key, queue_value))?;
    Ok(())
}

pub fn remove_from_retry_queue(rocks_sender: Sender, queue_key: String) -> Result<(), ExpectedError> {
    let _ = rocks_sender.send(RocksMsg::new(RocksMethod::Delete, queue_key, Value::Null))?;
    Ok(())
}

#[cfg(test)]
mod subscribe {
    use crate::libs::subscribe::load_task_from_json;

    #[test]
    fn load_task_test() {
        let sub_event = load_task_from_json("task/l2_block_tx.json", "optimism", "task:optimism", "l2_block_tx").unwrap();
        assert_eq!(sub_event.task_id, "task:optimism:l2_block_tx");
    }
}