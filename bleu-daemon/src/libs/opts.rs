use appbase::prelude::*;

use crate::error::error::ExpectedError;

pub fn string(key: &str) -> Result<String, ExpectedError> {
    let option_str = APP.options.value_of(key)
        .ok_or(ExpectedError::NoneError("argument is null!".to_string()))?;
    Ok(option_str.to_string())
}

pub fn u64(key: &str) -> Result<u64, ExpectedError> {
    let option_u64 = APP.options.value_of_t::<u64>(key)
        .ok_or(ExpectedError::NoneError("argument is null!".to_string()))?;
    Ok(option_u64)
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
