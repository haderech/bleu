use super::serde::get_string;
use crate::error::error::ExpectedError;
use serde_json::{Map, Value};

// pub async fn get(url: &str) -> Result<Map<String, Value>, ExpectedError> {
// 	let res = reqwest::get(url).await?;
// 	let status = res.status().clone();
// 	let body = res.text().await?;
// 	let parsed_body: Map<String, Value> = serde_json::from_str(body.as_str())?;

// 	if !status.is_success() {
// 		let error = get_string(&parsed_body, "error")?;
// 		return Err(ExpectedError::RequestError(error.to_string()))
// 	}
// 	Ok(parsed_body)
// }

pub async fn post(url: &str, req_body: &str) -> Result<Map<String, Value>, ExpectedError> {
	let req = String::from(req_body);
	let client = reqwest::Client::new();
	let res = client
		.post(url)
		.body(req)
		.header("Content-Type", "application/json")
		.send()
		.await?;
	let status = res.status().clone();
	let body = res.text().await?;
	let parsed_body: Map<String, Value> = serde_json::from_str(body.as_str())?;

	if !status.is_success() {
		let error = get_string(&parsed_body, "error")?;
		return Err(ExpectedError::RequestError(error.to_string()))
	}
	Ok(parsed_body)
}
