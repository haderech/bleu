use crate::{error::error::ExpectedError, libs::postgres::postgres_type};
use jsonrpc_core::Value;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PostgresSchema {
	pub schema_name: String,
	pub attributes: Vec<Attribute>,
	pub create_table: String,
	pub create_index: Vec<String>,
	pub insert_query: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Attribute {
	pub name: String,
	pub description: String,
	type_: String,
	max_length: Option<u32>,
	nullable: bool,
}

impl PostgresSchema {
	pub fn new(schema_name: String, values: &Value) -> Result<Self, ExpectedError> {
		let map = values.as_object().ok_or(ExpectedError::ParsingError(format!(
			"invalid value type; schema: {schema_name}"
		)))?;
		let raw_attributes = map
			.get("attributes")
			.ok_or(ExpectedError::ParsingError(format!(
				"attributes does not exist; schema: {schema_name}"
			)))?
			.as_object()
			.ok_or(ExpectedError::ParsingError(format!(
				"invalid attributes type; schema: {schema_name}"
			)))?;

		let mut attributes: Vec<Attribute> = Vec::new();
		for (key, value) in raw_attributes {
			let parsed_value = value.as_object().ok_or(ExpectedError::ParsingError(format!(
				"invalid attribute; schema: {schema_name}, attribute: {key}"
			)))?;
			let size = match parsed_value.get("maxLength") {
				None => None,
				Some(size) => {
					let size = size.as_u64().ok_or(ExpectedError::ParsingError(format!(
						"invalid size type; schema: {schema_name}, attribute: {key}"
					)))?;
					Some(size as u32)
				},
			};
			let description = match parsed_value.get("description") {
				None => key.clone(),
				Some(description) => description
					.as_str()
					.ok_or(ExpectedError::ParsingError(format!(
						"invalid description type; schema: {schema_name}, attribute: {key}"
					)))?
					.to_string(),
			};
			let ty = parsed_value.get("type").ok_or(ExpectedError::ParsingError(format!(
				"type does not exist; schema: {schema_name}, attribute: {key}"
			)))?;
			let (type_, nullable) = match ty {
				Value::Array(v) => {
					let v_str: Vec<String> =
						v.iter().map(|it| it.as_str().unwrap().to_string()).collect();
					if v_str.len() > 2 {
						return Err(ExpectedError::ParsingError(format!(
							"type array size cannot be bigger than 2; schema: {schema_name}, attribute: {key}"
						)))
					} else if v_str.len() > 1 && v_str[1] != "null" {
						return Err(ExpectedError::ParsingError(format!(
							"second value of types must be null; schema: {schema_name}, attribute: {key}"
						)))
					} else if v_str.len() > 1 && v_str[1] == "null" {
						(v_str.get(0).unwrap().clone(), true)
					} else {
						(v_str.get(0).unwrap().clone(), false)
					}
				},
				Value::String(v) => (v.clone(), false),
				_ =>
					return Err(ExpectedError::ParsingError(format!(
						"invalid type; schema: {schema_name}, attribute: {key}"
					))),
			};

			let attribute =
				Attribute { name: key.clone(), description, type_, max_length: size, nullable };
			attributes.push(attribute);
		}

		let uniques = map
			.get("uniques")
			.ok_or(ExpectedError::ParsingError(format!(
				"uniques does not exist; schema: {schema_name}"
			)))?
			.as_array()
			.ok_or(ExpectedError::ParsingError(format!(
				"invalid uniques; schema: {schema_name}"
			)))?;
		let indexes = map
			.get("indexes")
			.ok_or(ExpectedError::ParsingError(format!(
				"indexes does not exist; schema: {schema_name}"
			)))?
			.as_array()
			.ok_or(ExpectedError::ParsingError(format!(
				"invalid indexes; schema: {schema_name}"
			)))?;
		let create_table = Self::create_table(schema_name.clone(), &attributes, uniques);
		let create_index = Self::create_index(schema_name.clone(), indexes);
		let insert_query = Self::insert_query(schema_name.clone(), &attributes);

		Ok(PostgresSchema {
			schema_name: schema_name.clone(),
			attributes,
			create_table,
			create_index,
			insert_query,
		})
	}

	fn create_table(
		schema_name: String,
		attributes: &Vec<Attribute>,
		uniques: &Vec<Value>,
	) -> String {
		let mut query_line: Vec<String> = Vec::new();
		query_line.push(format!("{}_id serial8", schema_name));
		for attribute in attributes.iter() {
			let ty = postgres_type(&attribute.type_).unwrap();
			if attribute.max_length.is_none() {
				query_line.push(format!(
					"{} {} {}",
					attribute.name,
					ty,
					Self::null_or_not(attribute.nullable)
				));
			} else {
				query_line.push(format!(
					"{} {}({}) {}",
					attribute.name,
					ty,
					attribute.max_length.unwrap(),
					Self::null_or_not(attribute.nullable)
				));
			}
		}
		query_line.push(format!("CONSTRAINT {schema_name}_pk PRIMARY KEY ({schema_name}_id)"));

		for raw_keys in uniques.iter() {
			let unique_vec: Vec<String> = raw_keys
				.as_array()
				.unwrap()
				.iter()
				.map(|v| v.as_str().unwrap().to_string())
				.collect();
			let unique_name = format!("{}_{}_un", schema_name, unique_vec.join("_"));
			query_line.push(format!(
				"CONSTRAINT {} UNIQUE ({})",
				unique_name,
				unique_vec.join(", ")
			));
		}
		let full_query = query_line.join(", ");
		format!("CREATE TABLE {} ({})", schema_name, full_query)
	}

	fn create_index(schema_name: String, indexes: &Vec<Value>) -> Vec<String> {
		let mut index_query = Vec::new();
		for raw_keys in indexes.iter() {
			let index_vec: Vec<String> = raw_keys
				.as_array()
				.unwrap()
				.iter()
				.map(|v| v.as_str().unwrap().to_string())
				.collect();
			let index_name = format!("{}_{}_idx", schema_name, index_vec.join("_"));
			index_query.push(format!(
				"CREATE INDEX {} ON {} USING btree ({})",
				index_name,
				schema_name,
				index_vec.join(", ")
			));
		}
		index_query
	}

	fn insert_query(schema_name: String, attributes: &Vec<Attribute>) -> String {
		let mut column_vec = Vec::new();
		let mut value_vec = Vec::new();
		for attribute in attributes.iter() {
			column_vec.push(attribute.name.clone());
			value_vec.push(format!("${}$", attribute.description.clone()));
		}
		let columns = column_vec.join(", ");
		let values = value_vec.join(", ");

		format!("INSERT INTO {} ({}) VALUES ({})", schema_name, columns, values)
	}

	fn null_or_not(nullable: bool) -> String {
		if nullable {
			"NULL".to_string()
		} else {
			"NOT NULL".to_string()
		}
	}
}

#[cfg(test)]
mod postgres_test {
	use crate::types::postgres::PostgresSchema;
	use serde_json::Value;
	use std::{collections::HashMap, fs};

	#[test]
	fn create_table_test() {
		let json_str = fs::read_to_string("schema/ethereum.json").unwrap();
		let json_schema: Value = serde_json::from_str(json_str.as_str()).unwrap();
		let schema_map = json_schema.as_object().unwrap();

		let mut result_map = HashMap::new();
		for (schema_name, values) in schema_map {
			let schema = PostgresSchema::new(schema_name.clone(), values).unwrap();
			result_map.insert(schema_name.clone(), schema);
		}
		assert_eq!(result_map.len(), 4);
	}
}
