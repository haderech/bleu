use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::error::error::ExpectedError;
use crate::libs::serde::find_value;
use crate::plugin::postgres::Pool;
use crate::types::postgres::PostgresSchema;

pub fn convert_type(_type: String, max_length: Option<u32>) -> Result<String, ExpectedError> {
    let converted = if _type == "string" {
        "varchar"
    } else if _type == "integer" {
        if max_length.is_some() && max_length.unwrap() <= 11 {
            "int4"
        } else {
            "int8"
        }
    } else if _type == "number" {
        "float8"
    } else if _type == "boolean" {
        "boolean"
    } else if _type == "object" {
        "json"
    } else if _type == "array" {
        "varchar"
    } else {
        return Err(ExpectedError::TypeError(String::from("unsupported type!")));
    };
    Ok(String::from(converted))
}

pub fn create_table(pool: Pool, schema_map: &HashMap<String, PostgresSchema>) {
    let mut client = pool.get().unwrap();
    for (_, schema) in schema_map.iter() {
        if let Err(err) = client.execute(schema.create_table.as_str(), &[]) {
            log::warn!("{}", err.to_string());
        }
        for create_index in schema.create_index.iter() {
            if let Err(err) = client.execute(create_index.as_str(), &[]) {
                log::warn!("{}", err.to_string());
            }
        }
    }
}

pub fn insert_value(pool: Pool, schema: &PostgresSchema, values: &Map<String, Value>) -> Result<(), ExpectedError> {
    let mut client = pool.get().unwrap();
    let value_names = schema.attributes.iter().map(|attribute| { attribute.description.clone() }).collect::<Vec<String>>();
    let insert_query = create_insert_query(&schema.insert_query, value_names, values)?;
    let _ = client.execute(insert_query.as_str(), &[])?;
    Ok(())
}

fn create_insert_query(insert_query: &String, value_names: Vec<String>, values: &Map<String, Value>) -> Result<String, ExpectedError> {
    let mut temp_query = insert_query.clone();
    for value_name in value_names.iter() {
        let to_value = get_query_value(&values, value_name);
        let from = format!("${}$", value_name);
        temp_query = temp_query.replace(&from, &to_value);
    }
    Ok(temp_query)
}

pub fn get_query_value(values: &Map<String, Value>, target_name: &str) -> String {
    let value = find_value(values, target_name);
    match value {
        Value::Null => String::from("null"),
        Value::String(s) => format!("'{}'", s),
        _ => value.to_string(),
    }
}

#[cfg(test)]
mod postgres {
    use serde_json::{json, Map, Value};

    use crate::libs::postgres::{create_insert_query, get_query_value};

    #[test]
    fn create_insert_query_test() {
        let insert_query = String::from("INSERT INTO test (a, b, c) VALUES ($a$, $b$, $c$)");
        let value_names = vec![String::from("a"), String::from("b"), String::from("c")];
        let mut values = Map::new();
        values.insert(String::from("a"), Value::String(String::from("x")));
        values.insert(String::from("b"), Value::String(String::from("y")));
        values.insert(String::from("c"), Value::Bool(false));

        let created_insert_query = create_insert_query(&insert_query, value_names, &values).unwrap();
        assert_eq!("INSERT INTO test (a, b, c) VALUES ('x', 'y', false)", created_insert_query);
    }

    #[test]
    fn get_query_value_test() {
        let mut values = Map::new();
        values.insert(String::from("a"), Value::String(String::from("x")));
        values.insert(String::from("b"), Value::Bool(false));
        values.insert(String::from("c"), json!(1));

        let a = get_query_value(&values, "a");
        let b = get_query_value(&values, "b");
        let c = get_query_value(&values, "c");

        assert_eq!("'x'", a);
        assert_eq!("false", b);
        assert_eq!("1", c);
    }
}