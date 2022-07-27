use std::collections::HashMap;

use serde_json::{Map, Value};

use crate::error::error::ExpectedError;
use crate::libs::serde::{find_value, find_value_by_path};
use crate::plugin::postgres::Pool;
use crate::types::postgres::PostgresSchema;

pub fn convert_type(_type: String) -> Result<String, ExpectedError> {
  let converted = if _type == "string" {
    "varchar"
  } else if _type == "integer" {
    "bigint"
  } else if _type == "number" {
    "double precision"
  } else if _type == "boolean" {
    "boolean"
  } else if _type == "object" {
    "jsonb"
  } else if _type == "array" {
    "varchar"
  } else {
    return Err(ExpectedError::TypeError(format!("{} is not unsupported type. cv-damon only supports string, integer, number, boolean, object and array.", _type)));
  };
  Ok(String::from(converted))
}

pub fn create_table(pool: Pool, schema_map: &HashMap<String, PostgresSchema>) -> Result<(), r2d2_postgres::postgres::Error> {
  let mut client = pool.get().unwrap();
  for (_, schema) in schema_map.iter() {
    if let Err(err) = client.execute(schema.create_table.as_str(), &[]) {
      let _ = error_handler(err)?;
    }
    for create_index in schema.create_index.iter() {
      if let Err(err) = client.execute(create_index.as_str(), &[]) {
        let _ = error_handler(err)?;
      }
    }
  }
  Ok(())
}

pub fn error_handler(err: r2d2_postgres::postgres::Error) -> Result<(), r2d2_postgres::postgres::Error> {
  let err_str = err.to_string();
  if err_str.contains("already exists") {
    log::warn!("{}", err_str);
    Ok(())
  } else {
    log::error!("{}", err_str);
    Err(err)
  }
}

pub fn insert_value(pool: Pool, schema: &PostgresSchema, values: &mut Map<String, Value>, version: i64) -> Result<(), ExpectedError> {
  values.insert("version".to_string(), Value::from(version));
  let mut client = pool.get().unwrap();
  let value_names = schema.attributes.iter().map(|attribute| { attribute.description.clone() }).collect::<Vec<String>>();
  let insert_query = create_insert_query(&schema.insert_query, &schema.values_format, value_names, values)?;
  log::debug!("{}", insert_query);
  let _ = client.execute(insert_query.as_str(), &[])?;
  Ok(())
}

pub fn bulk_insert_value(pool: Pool, schema: &PostgresSchema, values_vec: &mut Vec<Value>, version: i64) -> Result<(), ExpectedError> {
  let mut client = pool.get().unwrap();
  let value_names = schema.attributes.iter().map(|attribute| { attribute.description.clone() }).collect::<Vec<String>>();
  let insert_query = create_bulk_insert_query(&schema.insert_query, &schema.values_format, value_names, values_vec, version)?;
  log::debug!("{}", insert_query);
  let _ = client.execute(insert_query.as_str(), &[])?;
  Ok(())
}

fn create_insert_query(insert_query: &String, values_format: &String, value_names: Vec<String>, values: &Map<String, Value>) -> Result<String, ExpectedError> {
  let mut values_format = values_format.clone();
  for value_name in value_names.iter() {
    let to = get_query_value(&values, value_name);
    let from = format!("${}$", value_name);
    values_format = values_format.replace(&from, &to);
  }
  Ok(format!("{} {}", insert_query, values_format))
}

fn create_bulk_insert_query(insert_query: &String, values_format: &String, value_names: Vec<String>, values_vec: &mut Vec<Value>, version: i64) -> Result<String, ExpectedError> {
  let mut values_parts: Vec<String> = Vec::new();
  for raw_values in values_vec {
    let mut temp_format = values_format.clone();
    let values = raw_values.as_object_mut().unwrap();
    values.insert("version".to_string(), Value::from(version));
    for value_name in value_names.iter() {
      let from = format!("${}$", value_name);
      let to = get_query_value(&values, value_name);
      temp_format = temp_format.replace(&from, &to);
    }
    values_parts.push(temp_format);
  }

  let values_part = values_parts.join(",");
  Ok(format!("{} {}", insert_query, values_part))
}

pub fn get_query_value(values: &Map<String, Value>, target_name: &str) -> String {
  let value = if target_name.contains(".") {
    find_value_by_path(values, target_name)
  } else {
    find_value(values, target_name)
  };
  match value {
    Value::Null => String::from("null"),
    Value::String(s) => {
      let esc = s.replace("'", "''");
      format!("'{}'", esc)
    },
    Value::Array(_) => {
      let org = value.to_string();
      let esc = org.replace("'", "''");
      format!("'{}'", esc)
    },
    Value::Object(_) => {
      let org = value.to_string();
      let esc = org.replace("'", "''");
      format!("'{}'", esc)
    },
    _ => value.to_string(),
  }
}
