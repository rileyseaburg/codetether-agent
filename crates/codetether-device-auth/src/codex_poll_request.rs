//! Token-poll request transport for Codex device authorization.
use crate::CodexDeviceCodeResponse;
use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::json;

pub(super) async fn post(
    client: &Client,
    url: &str,
    user_agent: &str,
    device: &CodexDeviceCodeResponse,
) -> Result<reqwest::Response> {
    let response = client
        .post(url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("User-Agent", user_agent)
        .json(&json!({
            "device_auth_id": device.device_auth_id,
            "user_code": device.user_code,
        }))
        .send()
        .await
        .with_context(|| format!("Failed to poll device authorization endpoint: {url}"))?;

    Ok(response)
}
