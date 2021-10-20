use std::collections::HashMap;
use std::fs;

use serde_json::Value;

use crate::error::error::ExpectedError;
use crate::libs::opts::opt_to_result;
use crate::types::subscribe::SubscribeEvent;

pub fn load_task(file_path: &str, chain: &str, prefix: &str) -> HashMap<String, SubscribeEvent> {
    match load_task_from_json(file_path, chain, prefix) {
        Ok(sub_events) => sub_events,
        Err(_) => HashMap::new()
    }
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
    use crate::libs::subscribe::load_task;

    #[test]
    fn load_task_test() {
        let sub_events = load_task("task/optimism.json", "optimism", "task:optimism");
        for (task_id, sub_event) in sub_events.iter() {
            println!("task_id={}, sub_event={:?}", task_id, sub_event);
        }
        assert_eq!(sub_events.len(), 4);
    }
}