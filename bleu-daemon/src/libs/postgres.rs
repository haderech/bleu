use crate::error::error::ExpectedError;

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