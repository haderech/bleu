use serde_json::Value;

use crate::error::error::ExpectedError;
use crate::libs::serde::get_type;

pub fn verify(params: &Vec<Value>) -> Result<(), ExpectedError> {
    for value in params {
        if get_type(value) != "string" {
            return Err(ExpectedError::TypeError("params must be string array!".to_string()));
        }
    }
    Ok(())
}
