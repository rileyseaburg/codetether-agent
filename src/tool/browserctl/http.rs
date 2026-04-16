//! HTTP transport helpers for browserctl requests.

use anyhow::{Context, Result};
use serde_json::{Value, json};

pub(super) async fn get(
    client: &reqwest::Client,
    base_url: &str,
    path: &str,
    token: Option<&str>,
) -> Result<(u16, Value)> {
    let mut request = client.get(format!("{base_url}{path}"));
    if let Some(token) = token {
        request = request.bearer_auth(token);
    }
    let response = request.send().await.context("browserctl GET failed")?;
    let status = response.status().as_u16();
    let body = response.text().await.unwrap_or_default();
    let parsed = serde_json::from_str::<Value>(&body).unwrap_or_else(|_| json!({"raw": body}));
    Ok((status, parsed))
}

pub(super) async fn post(
    client: &reqwest::Client,
    base_url: &str,
    path: &str,
    token: Option<&str>,
    payload: Value,
) -> Result<(u16, Value)> {
    let mut request = client.post(format!("{base_url}{path}")).json(&payload);
    if let Some(token) = token {
        request = request.bearer_auth(token);
    }
    let response = request.send().await.context("browserctl POST failed")?;
    let status = response.status().as_u16();
    let body = response.text().await.unwrap_or_default();
    let parsed = serde_json::from_str::<Value>(&body).unwrap_or_else(|_| json!({"raw": body}));
    Ok((status, parsed))
}
