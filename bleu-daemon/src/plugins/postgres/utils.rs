use crate::{ libs::serde::find_value, plugins::postgres::Pool,
	types::postgres::PostgresSchema,
};
use serde_json::{Map, Value};
use std::collections::HashMap;
use self::error::PostgresError;


pub fn sql_type(name: &str) -> Result<String, PostgresError> {
	let sql_type = match name {
		"string" => "varchar",
		"integer" => "bigint",
		"number" => "double precision",
		"object" => "json",
		"array" => "json",
		_ => return Err(PostgresError::UnsupportedType(format!("unsupported type: {name}")))
	};
	Ok(String::from(sql_type))
}

pub fn create_table(
	pool: Pool,
	schemas: &HashMap<String, PostgresSchema>,
) -> Result<(), PostgresError> {
	let mut client = pool.get().map_err(|e| PostgresError::Connection(e.to_string()))?;
	let mut queries: Vec<String> = Vec::new();
	let mut tables = schemas.iter().map(|(_, schema) | schema.create_table.clone()).collect::<Vec<String>>();
	let mut indices = schemas.iter().flat_map(|(_, schema) | schema.create_index).collect::<Vec<String>>();
	queries.append(&mut tables);
	queries.append(&mut indices);

	for query in queries {
		if let Err(e) = client.execute(query.as_str(), &[]) {
			if !e.to_string().contains("already exists") {
				return Err(PostgresError::ExecutedFailed(e.to_string()))
			}
		}
	}
	Ok(())
}

pub fn insert_value(
	pool: Pool,
	schema: &PostgresSchema,
	values: &Map<String, Value>,
) -> Result<(), ExpectedError> {
	let mut client = pool.get().unwrap();
	let value_names = schema
		.attributes
		.iter()
		.map(|attribute| attribute.description.clone())
		.collect::<Vec<String>>();
	let insert_query = create_insert_query(&schema.insert_query, value_names, values)?;
	let _ = client.execute(insert_query.as_str(), &[])?;
	Ok(())
}

fn create_insert_query(
	insert_query: &String,
	value_names: Vec<String>,
	values: &Map<String, Value>,
) -> Result<String, ExpectedError> {
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
		Value::Array(_) => format!("'{}'", value.to_string()),
		Value::Object(_) => format!("'{}'", value.to_string()),
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

		let created_insert_query =
			create_insert_query(&insert_query, value_names, &values).unwrap();
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
