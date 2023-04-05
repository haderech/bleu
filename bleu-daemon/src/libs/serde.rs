use crate::error::error::ExpectedError;
use serde_json::{Map, Value};
use std::str::FromStr;

pub fn get_string(values: &Map<String, Value>, name: &str) -> Result<String, ExpectedError> {
	let value = values
		.get(name)
		.ok_or(ExpectedError::ParsingError(format!("{} does not exist.", name)))?
		.as_str()
		.ok_or(ExpectedError::ParsingError(format!("{} is not string.", name)))?
		.to_string();
	Ok(value)
}

pub fn get_string_vec(
	params: &Map<String, Value>,
	name: &str,
) -> Result<Vec<String>, ExpectedError> {
	let value = params
		.get(name)
		.ok_or(ExpectedError::ParsingError(format!("{} does not exist.", name)))?
		.as_array()
		.ok_or(ExpectedError::ParsingError(format!("{} is not vector.", name)))?
		.iter()
		.map(|item| item.as_str().unwrap().to_string())
		.collect();
	Ok(value)
}

pub fn get_object<'a>(
	values: &'a Map<String, Value>,
	name: &str,
) -> Result<&'a Map<String, Value>, ExpectedError> {
	let value = values
		.get(name)
		.ok_or(ExpectedError::ParsingError(format!("{} does not exist.", name)))?
		.as_object()
		.ok_or(ExpectedError::ParsingError(format!("{} is not object.", name)))?;
	Ok(value)
}

pub fn get_array<'a>(
	values: &'a Map<String, Value>,
	name: &str,
) -> Result<&'a Vec<Value>, ExpectedError> {
	let value = values
		.get(name)
		.ok_or(ExpectedError::ParsingError(format!("{} does not exist.", name)))?
		.as_array()
		.ok_or(ExpectedError::ParsingError(format!("{} is not array.", name)))?;
	Ok(value)
}

pub fn get_u64(values: &Map<String, Value>, name: &str) -> Result<u64, ExpectedError> {
	let value = values
		.get(name)
		.ok_or(ExpectedError::ParsingError(format!("{} does not exist.", name)))?
		.as_u64()
		.ok_or(ExpectedError::ParsingError(format!("{} is not u64.", name)))?;
	Ok(value)
}

pub fn find_value(values: &Map<String, Value>, name: &str) -> Value {
	if values.get(name).is_some() {
		values.get(name).unwrap().clone()
	} else {
		for (_, value) in values.iter() {
			match value {
				Value::Object(object) => {
					let value = find_value(object, name);
					if value.is_null() {
						continue
					} else {
						return value
					}
				},
				Value::Array(vector) =>
					for element in vector {
						if element.is_object() {
							let value = find_value(element.as_object().unwrap(), name);
							if value.is_null() {
								continue
							} else {
								return value
							}
						}
					},
				_ => {},
			}
		}
		Value::Null
	}
}

pub fn get_value_by_path<'a>(
	params: &'a Map<String, Value>,
	path: &'a str,
) -> Result<&'a Value, ExpectedError> {
	let split = path.split(".");
	if split.clone().count() == 0 {
		return Err(ExpectedError::InvalidError("path cannot be empty.".to_string()))
	}
	let mut params = params;
	let last = split.clone().last().unwrap();
	for name in split {
		if name == last {
			let target = params
				.get(name)
				.ok_or(ExpectedError::ParsingError(format!("{} does not exist.", name)))?;
			return Ok(target)
		} else {
			params = params
				.get(name)
				.ok_or(ExpectedError::ParsingError(format!("{} does not exist.", name)))?
				.as_object()
				.ok_or(ExpectedError::ParsingError(format!("{} is not object.", name)))?;
		}
	}
	Err(ExpectedError::NoneError(format!("value does not exist in the path. path={}", path)))
}

// pub fn get_type(value: &Value) -> String {
// 	let types = match value {
// 		Value::Null => "null",
// 		Value::Bool(_) => "bool",
// 		Value::Number(v) =>
// 			if v.is_u64() {
// 				"u64"
// 			} else if v.is_i64() {
// 				"i64"
// 			} else if v.is_f64() {
// 				"f64"
// 			} else {
// 				"number"
// 			},
// 		Value::String(_) => "string",
// 		Value::Array(_) => "array",
// 		Value::Object(_) => "object",
// 	};
// 	String::from(types)
// }

pub fn filter(values: &Map<String, Value>, filter: String) -> Result<bool, ExpectedError> {
	if filter.trim().is_empty() {
		return Ok(true)
	}
	let mut calc_vector: Vec<String> = Vec::new();
	let mut key_value = String::new();
	let filter_chars = filter.chars();
	for c in filter_chars {
		if c == '&' || c == '|' || c == '(' || c == ')' {
			if !key_value.trim().is_empty() {
				let calc_result = filter_value(values, &key_value)?;
				calc_vector.push(calc_result.to_string());
			}
			calc_vector.push(String::from(c));
			key_value = String::new();
		} else {
			key_value.push(c);
		}
	}
	if !key_value.trim().is_empty() {
		let calc_result = filter_value(values, &key_value)?;
		calc_vector.push(calc_result.to_string());
	}

	let mut bool_stack: Vec<bool> = Vec::new();
	let mut calc_stack: Vec<String> = Vec::new();
	for vec_item in calc_vector {
		if vec_item == ")" {
			while calc_stack.last().is_some() && calc_stack.last().unwrap() != "(" {
				if bool_stack.len() < 2 {
					return Err(ExpectedError::InvalidError("filter format error.".to_string()))
				}
				let calc_ret = filter_calc(&mut bool_stack, &mut calc_stack)?;
				bool_stack.push(calc_ret);
			}
			calc_stack.pop();
		} else {
			if vec_item == "(" || vec_item == "&" || vec_item == "|" {
				calc_stack.push(vec_item.clone());
			} else {
				bool_stack.push(bool::from_str(vec_item.as_str()).unwrap());
			}
		}
	}
	while !calc_stack.is_empty() {
		let calc_ret = filter_calc(&mut bool_stack, &mut calc_stack)?;
		bool_stack.push(calc_ret);
	}
	let ret = bool_stack
		.pop()
		.ok_or(ExpectedError::InvalidError("invalid filter condition.".to_string()))?;
	Ok(ret)
}

fn filter_value(values: &Map<String, Value>, key_value: &String) -> Result<bool, ExpectedError> {
	let mut split_kv = key_value.split("=");
	if split_kv.clone().count() != 2 {
		return Err(ExpectedError::TypeError(
			"invalid filter condition format. example='key=val'".to_string(),
		))
	}
	let key = split_kv.next().unwrap().trim();
	let value = split_kv.next().unwrap().trim();
	let found = if key.contains(".") {
		match get_value_by_path(values, key) {
			Ok(val) => val.clone(),
			Err(_) => Value::Null,
		}
	} else {
		find_value(values, key)
	};
	let found_val = match found {
		Value::String(s) => s,
		_ => found.to_string(),
	};
	Ok(value == found_val.as_str())
}

fn filter_calc(
	bool_stack: &mut Vec<bool>,
	calc_stack: &mut Vec<String>,
) -> Result<bool, ExpectedError> {
	let calc_op = calc_stack
		.pop()
		.ok_or(ExpectedError::InvalidError("invalid filter condition.".to_string()))?;
	let top = bool_stack
		.pop()
		.ok_or(ExpectedError::InvalidError("invalid filter condition.".to_string()))?;
	let second = bool_stack
		.pop()
		.ok_or(ExpectedError::InvalidError("invalid filter condition.".to_string()))?;
	if calc_op == "&" {
		Ok(top & second)
	} else {
		Ok(top | second)
	}
}

#[cfg(test)]
mod serde {
	use serde_json::{json, Map, Value};

	use crate::libs::serde;

	#[test]
	fn filter_success_test() {
		let mut test_map = Map::new();
		test_map.insert(String::from("key1"), Value::String(String::from("val1")));
		test_map.insert(String::from("key2"), json!({"sub_key1": "sub_val1"}));
		test_map.insert(String::from("key3"), json!(100));

		let ret = serde::filter(
			&test_map,
			String::from("(key1 = val1 & sub_key1 = sub_val1 & key3 =101) | key4=null | key3=101"),
		)
		.unwrap();
		assert_eq!(ret, true);
	}

	#[test]
	fn filter_fail_test() {
		let mut test_map = Map::new();
		test_map.insert(String::from("key1"), Value::String(String::from("val1")));
		test_map.insert(String::from("key2"), json!({"sub_key1": "sub_val1"}));
		test_map.insert(String::from("key3"), json!(100));
		test_map.insert(String::from("key4"), Value::String(String::from("not_null")));

		let ret = serde::filter(
			&test_map,
			String::from("(key1 = val1 & sub_key1 = sub_val1 & key3 =100) & key4=null"),
		)
		.unwrap();
		assert_eq!(ret, false);
	}
}
