use std::str::FromStr;

use appbase::prelude::*;
use serde::Deserialize;

use crate::error::error::ExpectedError;

pub fn get_value_str(key: &str) -> Result<String, ExpectedError> {
    let value_str = APP.options.value_of(key)
        .ok_or(ExpectedError::NoneError("argument is null!".to_string()))?;
    Ok(value_str.to_string())
}

pub fn get_value<T>(key: &str) -> Result<T, ExpectedError>
    where
        T: FromStr + Deserialize<'static>,
        <T as FromStr>::Err: std::fmt::Display
{
    let value_str = APP.options.value_of_t::<T>(key)
        .ok_or(ExpectedError::NoneError("argument is null!".to_string()))?;
    Ok(value_str)
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
