use appbase::prelude::*;

use crate::error::error::ExpectedError;

pub fn string(key: &str) -> Result<String, ExpectedError> {
    let string = APP.options.value_of(key)
        .ok_or(ExpectedError::NoneError("argument is null!".to_string()))?;
    Ok(string.to_string())
}

pub fn opt_to_result<T>(option: Option<T>) -> Result<T, ExpectedError> {
    match option {
        Some(t) => Ok(t),
        None => Err(ExpectedError::NoneError(String::from("value is none!")))
    }
}

pub fn opt_ref_to_result<T>(option: Option<&T>) -> Result<&T, ExpectedError> {
    match option {
        Some(t) => Ok(t),
        None => Err(ExpectedError::NoneError(String::from("value is none!")))
    }
}
