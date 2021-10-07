use std::fmt::Debug;

use jsonrpc_core::Value;

use crate::error::error::ExpectedError;
use crate::libs::postgres::convert_type;
use crate::libs::serde::{get_array, get_object};

#[derive(Clone, Debug)]
pub struct PostgresSchema {
    pub schema_name: String,
    pub attributes: Vec<Attribute>,
    pub create_table: String,
    pub create_index: Vec<String>,
    pub insert_query: String,
}

#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String,
    pub description: String,
    _type: String,
    max_length: Option<u32>,
    nullable: bool,
}

impl PostgresSchema {
    pub fn from(schema_name: String, values: &Value) -> Result<PostgresSchema, ExpectedError> {
        if !values.is_object() {
            return Err(ExpectedError::TypeError(String::from("input values is not object type!")));
        }
        let map = values.as_object().unwrap();
        let raw_attributes = get_object(map, "attributes")?;

        let mut attributes: Vec<Attribute> = Vec::new();
        for (key, value) in raw_attributes {
            let parsed_value = value.as_object().unwrap();
            let size = match parsed_value.get("maxLength") {
                None => None,
                Some(size) => Some(size.as_u64().unwrap() as u32)
            };
            let description = match parsed_value.get("description") {
                None => key.clone(),
                Some(description) => String::from(description.as_str().unwrap())
            };
            let type_value = match parsed_value.get("type") {
                None => return Err(ExpectedError::NoneError(String::from("schema attribute must include type!"))),
                Some(type_value) => type_value
            };
            let (_type, nullable) = match type_value {
                Value::Array(v) => {
                    let v_str: Vec<String> = v.iter().map(|it| { String::from(it.as_str().unwrap()) }).collect();
                    if v_str.len() > 2 {
                        return Err(ExpectedError::InvalidError(String::from("type array size cannot be bigger than 2!")));
                    }
                    if v_str.len() > 1 && v_str.get(1).unwrap() != "null" {
                        return Err(ExpectedError::InvalidError(String::from("second value of types must be null!")));
                    }
                    (v_str.get(0).unwrap().clone(), true)
                }
                Value::String(v) => (v.clone(), false),
                _ => return Err(ExpectedError::TypeError(String::from("type only can be string or array!")))
            };

            let attribute = Attribute {
                name: key.clone(),
                description,
                _type,
                max_length: size,
                nullable,
            };
            attributes.push(attribute);
        }

        let uniques = get_array(map, "uniques")?;
        let indexes = get_array(map, "indexes")?;
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

    fn create_table(schema_name: String, attributes: &Vec<Attribute>, uniques: &Vec<Value>) -> String {
        let mut query_line: Vec<String> = Vec::new();
        query_line.push(format!("{}_id serial4", schema_name));
        for attribute in attributes.iter() {
            let converted_type = convert_type(attribute._type.clone()).unwrap();
            if attribute.max_length.is_none() {
                query_line.push(format!("{} {} {}", attribute.name, converted_type, Self::null_or_not(attribute.nullable)));
            } else {
                query_line.push(format!("{} {}({}) {}", attribute.name, converted_type, attribute.max_length.unwrap(), Self::null_or_not(attribute.nullable)));
            }
        }
        query_line.push(format!("CONSTRAINT {schema_name}_pk PRIMARY KEY ({schema_name}_id)", schema_name = schema_name));

        for raw_keys in uniques.iter() {
            let unique_vec: Vec<String> = raw_keys.as_array().unwrap().iter().map(|v| { String::from(v.as_str().unwrap()) }).collect();
            let unique_name = format!("{}_{}_un", schema_name, unique_vec.join("_"));
            query_line.push(format!("CONSTRAINT {} UNIQUE ({})", unique_name, unique_vec.join(", ")));
        }
        let full_query = query_line.join(", ");
        format!("CREATE TABLE {} ({})", schema_name, full_query)
    }

    fn create_index(schema_name: String, indexes: &Vec<Value>) -> Vec<String> {
        let mut index_query = Vec::new();
        for raw_keys in indexes.iter() {
            let index_vec: Vec<String> = raw_keys.as_array().unwrap().iter().map(|v| { String::from(v.as_str().unwrap()) }).collect();
            let index_name = format!("{}_{}_idx", schema_name, index_vec.join("_"));
            index_query.push(format!("CREATE INDEX {} ON {} USING btree ({})", index_name, schema_name, index_vec.join(", ")));
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
            String::from("NULL")
        } else {
            String::from("NOT NULL")
        }
    }
}

#[cfg(test)]
mod postgres_test {
    use std::collections::HashMap;
    use std::fs;

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
        assert_eq!(result_map.len(), 2);
    }

    #[test]
    fn insert_query_test() {
        let json_str = fs::read_to_string("schema/ethereum.json").unwrap();
        let json_schema: Value = serde_json::from_str(json_str.as_str()).unwrap();
        let schema_map = json_schema.as_object().unwrap();

        let mut result_map = HashMap::new();
        for (schema_name, values) in schema_map {
            let schema = PostgresSchema::from(schema_name.clone(), values).unwrap();
            result_map.insert(schema_name.clone(), schema);
        }
        let selected_schema = result_map.get("eth_blocks").unwrap().to_owned();
        let created_insert_query = selected_schema.insert_query;
        assert_eq!(created_insert_query, "INSERT INTO eth_blocks (base_fee_per_gas, block_number, block_size, block_timestamp, difficulty, extra_data, gas_limit, gas_used, hash, is_forked, logs_bloom, miner, mix_hash, nonce, parent_hash, receipts_root, sha3_uncles, state_root, total_difficulty) VALUES ($baseFeePerGas$, $number$, $size$, $timestamp$, $difficulty$, $extraData$, $gasLimit$, $gasUsed$, $hash$, $is_forked$, $logsBloom$, $miner$, $mixHash$, $nonce$, $parentHash$, $receiptsRoot$, $sha3Uncles$, $stateRoot$, $totalDifficulty$)");
    }
}