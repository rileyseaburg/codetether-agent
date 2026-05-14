use crate::browser::{BrowserError, BrowserOutput};
use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;

pub(super) async fn send(
    method: &str,
    url: &str,
    headers: Option<HashMap<String, String>>,
    body: Option<String>,
) -> Result<BrowserOutput, BrowserError> {
    let mut builder = reqwest::Client::new().request(parse_method(method)?, url);
    for (key, value) in headers.unwrap_or_default() {
        builder = builder.header(key, value);
    }
    if let Some(body) = body {
        builder = builder.body(body);
    }
    let response = builder
        .send()
        .await
        .map_err(super::super::super::fetch::map)?;
    let status = response.status().as_u16();
    let text = response
        .text()
        .await
        .map_err(super::super::super::fetch::map)?;
    Ok(BrowserOutput::Json(json!({
        "ok": status < 400,
        "status": status,
        "text": text
    })))
}

fn parse_method(method: &str) -> Result<Method, BrowserError> {
    Method::from_bytes(method.as_bytes())
        .map_err(|error| BrowserError::OperationFailed(error.to_string()))
}
