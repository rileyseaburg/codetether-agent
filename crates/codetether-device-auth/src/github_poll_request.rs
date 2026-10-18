//! GitHub token-poll transport and response decoding.
use crate::{DeviceCodeResponse, error_text::truncate_body, github_types::AccessTokenResponse};
use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::json;

pub(super) async fn payload(
    client: &Client,
    url: &str,
    user_agent: &str,
    client_id: &str,
    device: &DeviceCodeResponse,
) -> Result<AccessTokenResponse> {
    let response = client
        .post(url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("User-Agent", user_agent)
        .json(&json!({
            "client_id": client_id,
            "device_code": device.device_code,
            "grant_type": "urn:ietf:params:oauth:grant-type:device_code",
        }))
        .send()
        .await
        .with_context(|| format!("Failed to poll token endpoint: {url}"))?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!(
            "Failed to exchange device code for access token ({}): {}",
            status,
            truncate_body(&body)
        );
    }

    let payload: AccessTokenResponse = response
        .json()
        .await
        .context("Failed to parse OAuth token response")?;

    Ok(payload)
}
