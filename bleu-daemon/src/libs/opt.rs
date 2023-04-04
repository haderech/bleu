use std::str::FromStr;

use appbase::prelude::*;
use serde::Deserialize;

use crate::error::error::ExpectedError;

pub fn get_value<T>(key: &str) -> Result<T, ExpectedError>
where
	T: FromStr + Deserialize<'static>,
	<T as FromStr>::Err: std::fmt::Display,
{
	let value = APP
		.options
		.value_of_t::<T>(key)
		.ok_or(ExpectedError::NoneError(format!("{} does not exist.", key)))?;
	Ok(value)
}
