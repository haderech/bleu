use regex::Regex;
use serde_json::{Map, Value};

use crate::error::error::ExpectedError;
use crate::libs::opt::opt_to_result;

pub fn hex_to_decimal(hex_str: String) -> Result<String, ExpectedError> {
    match is_hex_string(&hex_str) {
        true => {
            let prefix_removed = hex_str.trim_start_matches("0x");
            match primitive_types::U256::from_str_radix(prefix_removed, 16) {
                Ok(decimal_u256) => Ok(decimal_u256.to_string()),
                Err(err) => Err(ExpectedError::ParsingError(err.to_string()))
            }
        }
        false => Err(ExpectedError::InvalidError(format!("input value is not hex string! input={}", hex_str)))
    }
}

pub fn hex_to_decimal_converter(origin_map: &Map<String, Value>, target_keys: Vec<&str>) -> Result<Map<String, Value>, ExpectedError> {
    let mut cloned_map = origin_map.clone();
    for key in target_keys.into_iter() {
        if let true = cloned_map.contains_key(key) {
            let value = opt_to_result(cloned_map.get(key))?;
            if value.is_string() {
                let origin_string = String::from(opt_to_result(value.as_str())?);
                if is_hex_string(&origin_string) {
                    let converted = hex_to_decimal(origin_string)?;
                    cloned_map.insert(key.to_owned(), Value::String(converted));
                }
            }
        }
    }
    Ok(cloned_map)
}

fn is_hex_string(hex_str: &str) -> bool {
    let regex = Regex::new(r"^(0[xX])?[A-Fa-f0-9]+$").unwrap();
    regex.is_match(hex_str)
}

pub fn number_to_string_convert(origin_map: &Map<String, Value>, target_keys: Vec<&str>) -> Result<Map<String, Value>, ExpectedError> {
    let mut cloned_map = origin_map.clone();
    for key in target_keys.into_iter() {
        if let true = cloned_map.contains_key(key) {
            let value = opt_to_result(cloned_map.get(key))?;
            if value.is_number() {
                let converted_value = value.to_string();
                cloned_map.insert(key.to_owned(), Value::String(converted_value));
            }
        }
    }
    Ok(cloned_map)
}

#[cfg(test)]
mod number {
    use serde_json::{Map, Value};

    use crate::libs::convert::{hex_to_decimal, hex_to_decimal_converter, number_to_string_convert};

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

        let converted_map = hex_to_decimal_converter(&test_map, vec!["key1", "key3", "key4"]).unwrap();
        assert_eq!(converted_map.get("key1").unwrap(), "17");
        assert_eq!(converted_map.get("key2").unwrap(), "0x22");
        assert_eq!(converted_map.get("key3").unwrap(), "bleu-daemon");
        assert_eq!(converted_map.get("key4").unwrap().clone(), Value::Null);
    }

    #[test]
    fn number_to_string_convert_test() {
        let mut test_map = Map::new();
        test_map.insert(String::from("key1"), Value::from(1));
        test_map.insert(String::from("key2"), Value::String("a".to_string()));
        test_map.insert(String::from("key3"), Value::from(1));

        let converted_map = number_to_string_convert(&test_map, vec!["key1", "key2"]).unwrap();
        assert_eq!(converted_map.get("key1").unwrap(), "1");
        assert_eq!(converted_map.get("key2").unwrap(), "a");
        assert_eq!(converted_map.get("key3").unwrap().clone(), Value::from(1));
    }
}