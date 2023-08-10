use crate::error::error::ExpectedError;
use serde::{de::DeserializeOwned, Serialize};
use std::fs;

pub fn write<T>(dir: &str, name: &str, contents: &T) -> Result<(), ExpectedError>
where
	T: ?Sized + Serialize,
{
	let _ = fs::create_dir(dir);
	let contents = serde_json::to_string(contents)?;
	let path = format!("{dir}/{name}.json");
	fs::write(path, contents)?;
	Ok(())
}

pub fn read<'a, T>(dir: &str, name: &str) -> Result<T, ExpectedError>
where
	T: DeserializeOwned,
{
	let path = format!("{dir}/{name}.json");
	let contents = fs::read(path)?;
	Ok(serde_json::from_slice::<T>(&contents)?)
}
