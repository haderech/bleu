use serde_json::{Map, Value};

use crate::error::error::ExpectedError;
use crate::libs::serde::get_string;

pub fn get(url: &str) -> Result<Map<String, Value>, ExpectedError> {
    let res = reqwest::blocking::get(url)?;
    let status = res.status().clone();
    let body = res.text()?;
    let parsed_body: Map<String, Value> = serde_json::from_str(body.as_str())?;

    if !status.is_success() {
        let error = get_string(&parsed_body, "error")?;
        return Err(ExpectedError::RequestError(error));
    }
    Ok(parsed_body)
}

pub fn post(url: &str, req_body: &str) -> Result<Map<String, Value>, ExpectedError> {
    let req = String::from(req_body);
    let client = reqwest::blocking::Client::new();
    let res = client.post(url).body(req).header("Content-Type", "application/json").send()?;
    let status = res.status().clone();
    let body = res.text()?;
    let parsed_body: Map<String, Value> = serde_json::from_str(body.as_str())?;

    if !status.is_success() {
        let error = get_string(&parsed_body, "error")?;
        return Err(ExpectedError::RequestError(error));
    }
    Ok(parsed_body)
}

pub fn adjust_url(url: String) -> String {
    let mut tmp_url = url.clone();
    if !tmp_url.ends_with("/") {
        tmp_url += "/";
    }
    tmp_url.to_owned()
}

#[cfg(test)]
mod request {
    use crate::libs::request::adjust_url;

    #[test]
    fn adjust_url_test() {
        let example_url = adjust_url(String::from("https://example.com"));
        assert_eq!("https://example.com/", example_url);

        let example_url2 = adjust_url(String::from("https://example2.com/"));
        assert_eq!("https://example2.com/", example_url2);
    }
}