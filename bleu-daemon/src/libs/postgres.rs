use super::filedb;
use crate::{
	error::error::ExpectedError, libs::serde::find_value, plugin::postgres::Pool,
	types::postgres::PostgresSchema,
};
use serde_json::{Map, Value};
use std::collections::HashMap;

pub fn postgres_type(ty: &str) -> Result<String, ExpectedError> {
	let postgres_type = match ty {
		"string" => "varchar",
		"integer" => "bigint",
		"number" => "double precision",
		"boolean" => "boolean",
		"object" => "json",
		"array" => "varchar",
		_ => return Err(ExpectedError::UnsupportedType(format!("unsupported type; type: {}", ty))),
	};
	Ok(postgres_type.to_string())
}

pub fn create_table(
	pool: Pool,
	schema_map: &HashMap<String, PostgresSchema>,
) -> Result<(), r2d2_postgres::postgres::Error> {
	let mut client = pool.get().expect("failed to get connection from pool");
	for (_, schema) in schema_map.iter() {
		if let Err(e) = client.execute(schema.create_table.as_str(), &[]) {
			if !e.to_string().contains("already exists") {
				return Err(e)
			}
		}
		for create_index in schema.create_index.iter() {
			if let Err(e) = client.execute(create_index.as_str(), &[]) {
				if !e.to_string().contains("already exists") {
					return Err(e)
				}
			}
		}
	}
	Ok(())
}

pub fn insert(
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

pub fn load_schema(
	schema_files: Vec<&str>,
) -> Result<HashMap<String, PostgresSchema>, ExpectedError> {
	let mut schemas = HashMap::<String, PostgresSchema>::new();
	for name in schema_files {
		let schema = filedb::read::<Map<String, Value>>("schema", name)
			.expect(format!("failed to load schema; schema: {name}").as_str());
		let schema = schema
			.iter()
			.map(|(name, schema)| {
				let schema = PostgresSchema::new(name.clone(), schema)
					.expect(format!("failed to create postgres schema; schema: {name}").as_str());
				(name.clone(), schema)
			})
			.collect::<HashMap<String, PostgresSchema>>();
		schemas.extend(schema);
	}
	Ok(schemas)
}

#[cfg(test)]
mod postgres {
	use crate::libs::postgres::{create_insert_query, get_query_value};
	use serde_json::{json, Map, Value};

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
