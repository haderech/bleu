use serde_json::Value;

use super::error::PostgresError;


#[derive(Clone, Debug)]
pub struct PostgresSchema {
	pub schema_name: String,
	pub attributes: Vec<Attribute>,
	pub create_query: CreateQuery,
	pub insert_quert: InsertQuery,
}

pub struct CreateQuery {
	pub table: String,
	pub indexes: Vec<String>,
}

pub struct InsertQuery {
	pub query: String,
	pub values: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct Attribute {
	pub name: String,
	pub description: String,
	type_: String,
	max_length: Option<u32>,
	nullable: bool,
}

impl PostgresSchema {
	pub fn new(name: &str, schema: &Value) -> Result<PostgresSchema, PostgresError> {
		let schema = schema.as_object().expect("invalid schema data");
		let attributes = schema.get("attributes")
			.expect("not exist attributes")
			.as_object()
			.expect("invalid attributes");

		let attributes = attributes.iter()
			.map(|(name, attribute)| (name.clone(), attribute.as_object().expect(format!("invalid attribute: {}", name).as_str())))
			.map(|(name, attribute)| {
			let (type_, nullable) = match attribute.get("type").expect(format!("not exist type; {}", name).as_str()) {
				Value::Array(types) => {
					let types = types.iter()
					.map(|ty| ty.as_str().map(ToString).expect(format!("invalid type; {}", name).as_str()))
					.collect::<Vec<String>>();

					assert!(types.len() <= 2, "type array size cannot be bigger than 2; {}", name);
					assert!((types.len() == 2 && types[1] == "null") || types.len() == 1, "invalid type; {}", name);
					(types[0], true)
				},
				Value::String(ty) => (ty.clone(), false),
				_ => panic!("type only can be string or array; {}", name)
			};

			Attribute {
				name,
				description: attribute.get("description").map(|desc| desc.as_str()).map(ToString).unwrap_or(name),
				type_,
				max_length: attribute.get("maxLength")
					.map(|max_len| max_len.as_u64().expect(format!("invalid maxLength type: {}", name).as_str()))
					.map(|max_len| max_len as u32),
				nullable,
			}
		}).collect::<Vec<Attribute>>();

		let uniques = schema.get("uniques")
			.expect(format!("not exist uniques: {}", name).as_str())
			.as_array()
			.expect(format!("invalid uniques type: {}", name).as_str());
		let indexes = schema.get("indexes")
			.expect(format!("not exist indexes: {}", name).as_str())
			.as_array()
			.expect(format!("invalid indexes type: {}", name).as_str());

		let create_query = Self::create_query(name.clone(), &attributes, uniques, indexes);
		let create_index_queries = Self::create_index(schema_name.clone(), indexes);
		let insert_query = Self::insert_query(schema_name.clone(), &attributes);
		let values =

		Ok(PostgresSchema {
			schema_name: schema_name.clone(),
			attributes,
			create_table_query,
			create_index_queries,
			insert_query,
    	values: todo!(),
		})
		}


	}

	fn create_query(
		name: String,
		attributes: &Vec<Attribute>,
		uniques: &Vec<Value>,
		indexes: &Vec<Value>,
	) -> CreateQuery {
		let mut query_line: Vec<String> = Vec::new();
		query_line.push(format!("{}_id serial8", name));
		for attribute in attributes.iter() {
			let converted_type = convert_type(attribute.type_.clone()).unwrap();
			if attribute.max_length.is_none() {
				query_line.push(format!(
					"{} {} {}",
					attribute.name,
					converted_type,
					Self::null_or_not(attribute.nullable)
				));
			} else {
				query_line.push(format!(
					"{} {}({}) {}",
					attribute.name,
					converted_type,
					attribute.max_length.unwrap(),
					Self::null_or_not(attribute.nullable)
				));
			}
		}
		query_line.push(format!(
			"CONSTRAINT {name}_pk PRIMARY KEY ({name}_id)",
			name = name
		));

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
		format!("CREATE TABLE {} ({})", name, full_query)
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
	use std::{collections::HashMap, fs};

	use serde_json::Value;

	use crate::types::postgres::PostgresSchema;

	#[test]
	fn create_table_test() {
		let json_str = fs::read_to_string("schema/ethereum.json").unwrap();
		let json_schema: Value = serde_json::from_str(json_str.as_str()).unwrap();
		let schema_map = json_schema.as_object().unwrap();

		let mut result_map = HashMap::new();
		for (schema_name, values) in schema_map {
			let schema = PostgresSchema::from(schema_name.clone(), values).unwrap();
			result_map.insert(schema_name.clone(), schema);
		}
		assert_eq!(result_map.len(), 1);
	}
}
