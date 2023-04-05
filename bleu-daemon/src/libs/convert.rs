use crate::error::error::ExpectedError;
use regex::Regex;
use serde_json::{Map, Value};

pub fn hex_to_decimal(hex_str: String) -> Result<String, ExpectedError> {
	match is_hex_string(&hex_str) {
		true => {
			let prefix_removed = hex_str.trim_start_matches("0x");
			match primitive_types::U256::from_str_radix(prefix_removed, 16) {
				Ok(decimal_u256) => Ok(decimal_u256.to_string()),
				Err(err) => Err(ExpectedError::ParsingError(err.to_string())),
			}
		},
		false => Err(ExpectedError::InvalidError(format!(
			"input value is not hex string. input={}",
			hex_str
		))),
	}
}

pub fn hex_to_decimal_converter(
	origin: &Map<String, Value>,
	keys: Vec<&str>,
) -> Result<Map<String, Value>, ExpectedError> {
	let mut cloned = origin.clone();
	for key in keys.into_iter() {
		if let true = cloned.contains_key(key) {
			let value = cloned
				.get(key)
				.ok_or(ExpectedError::ParsingError(format!("{} does not exist.", key)))?;
			if value.is_string() {
				let str = value.as_str().unwrap().to_string();
				if is_hex_string(&str) {
					let converted = hex_to_decimal(str)?;
					cloned.insert(key.to_owned(), Value::String(converted));
				}
			}
		}
	}
	Ok(cloned)
}

fn is_hex_string(hex_str: &str) -> bool {
	let regex = Regex::new(r"^(0[xX])?[A-Fa-f0-9]+$").unwrap();
	regex.is_match(hex_str)
}

// pub fn number_to_string_convert(
// 	origin: &Map<String, Value>,
// 	keys: Vec<&str>,
// ) -> Result<Map<String, Value>, ExpectedError> {
// 	let mut cloned = origin.clone();
// 	for key in keys.into_iter() {
// 		if let true = cloned.contains_key(key) {
// 			let value = cloned
// 				.get(key)
// 				.ok_or(ExpectedError::ParsingError(format!("{} does not exist.", key)))?;
// 			if value.is_number() {
// 				cloned.insert(key.to_owned(), Value::String(value.to_string()));
// 			}
// 		}
// 	}
// 	Ok(cloned)
// }

#[cfg(test)]
mod number {
	use serde_json::{Map, Value};

	use crate::libs::convert::{hex_to_decimal, hex_to_decimal_converter};

	#[test]
	fn hex_to_decimal_test() {
		let decimal_str = hex_to_decimal(String::from("0x16345785d8a0000")).unwrap();
		assert_eq!("100000000000000000", decimal_str);

		let decimal_str = hex_to_decimal(String::from("16345785d8a0000")).unwrap();
		assert_eq!("100000000000000000", decimal_str);
	}

	#[test]
	fn hex_to_decimal_fail_test() {
		let result = hex_to_decimal(String::from("0x16345785d8a0000z"));
		assert!(result.is_err());

		let result = hex_to_decimal(String::from("xx16345785d8a0000"));
		assert!(result.is_err());
	}

	#[test]
	fn hex_to_decimal_converter_test() {
		let mut test_map = Map::new();
		test_map.insert(String::from("key1"), Value::String(String::from("0x11")));
		test_map.insert(String::from("key2"), Value::String(String::from("0x22")));
		test_map.insert(String::from("key3"), Value::String(String::from("bleu-daemon")));
		test_map.insert(String::from("key4"), Value::Null);

		let converted_map =
			hex_to_decimal_converter(&test_map, vec!["key1", "key3", "key4"]).unwrap();
		assert_eq!(converted_map.get("key1").unwrap(), "17");
		assert_eq!(converted_map.get("key2").unwrap(), "0x22");
		assert_eq!(converted_map.get("key3").unwrap(), "bleu-daemon");
		assert_eq!(converted_map.get("key4").unwrap().clone(), Value::Null);
	}

	// #[test]
	// fn number_to_string_convert_test() {
	// 	let mut test_map = Map::new();
	// 	test_map.insert(String::from("key1"), Value::from(1));
	// 	test_map.insert(String::from("key2"), Value::String("a".to_string()));
	// 	test_map.insert(String::from("key3"), Value::from(1));

	// 	let converted_map = number_to_string_convert(&test_map, vec!["key1", "key2"]).unwrap();
	// 	assert_eq!(converted_map.get("key1").unwrap(), "1");
	// 	assert_eq!(converted_map.get("key2").unwrap(), "a");
	// 	assert_eq!(converted_map.get("key3").unwrap().clone(), Value::from(1));
	// }
}
