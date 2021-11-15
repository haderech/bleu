use std::sync::Arc;

use appbase::prelude::*;
use jsonrpc_core::serde_from_str;
use rocksdb::{DBWithThreadMode, SingleThreaded};
use serde::Serialize;
use serde_json::{json, Map, Value};

use crate::error::error::ExpectedError;
use crate::libs::rocks;
use crate::plugin::rocks::{RocksMethod, RocksMsg};

type RocksDB = Arc<DBWithThreadMode<SingleThreaded>>;

pub fn get_static(rocksdb: &RocksDB, key: &str) -> Result<Value, ExpectedError> {
    let result = rocksdb.get(key.as_bytes())?;
    let result_value = match result {
        None => Value::Null,
        Some(value) => Value::Object(serde_json::from_str(String::from_utf8(value)?.as_str())?),
    };
    Ok(result_value)
}

pub fn get_by_prefix_static(rocksdb: &RocksDB, prefix: &str) -> Value {
    let mut iter = rocksdb.raw_iterator();
    iter.seek(prefix.as_bytes());
    let mut result: Vec<Value> = Vec::new();
    while iter.valid() && rocks::deserialize(iter.key().unwrap()).starts_with(prefix) {
        let value: Map<String, Value> = serde_from_str(rocks::deserialize(iter.value().unwrap()).as_str()).unwrap();
        result.push(Value::Object(value));
        iter.next();
    }
    Value::Array(result)
}

pub fn deserialize(val: &[u8]) -> String {
    String::from_utf8(val.to_vec()).unwrap()
}

pub fn save<T>(rocks_sender: &Sender, key: String, value: T) -> Result<(), ExpectedError>
    where
        T: Serialize
{
    let _ = rocks_sender.send(RocksMsg::new(RocksMethod::Put, key, Value::String(json!(value).to_string())))?;
    Ok(())
}

pub fn delete(rocks_sender: &Sender, key: String) -> Result<(), ExpectedError> {
    let _ = rocks_sender.send(RocksMsg::new(RocksMethod::Delete, key, Value::Null))?;
    Ok(())
}

#[cfg(test)]
mod serialize_test {
    use crate::libs::rocks::deserialize;

    #[test]
    fn deserialize_test() {
        let serialized = "test".as_bytes();
        let deserialized = deserialize(serialized);
        assert_eq!(deserialized.as_str(), "test");
    }
}
