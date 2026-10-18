//! Device authorization github_request responsibilities.
use crate::{DeviceCodeResponse, error_text::truncate_body};
use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::json;

/// Request a GitHub-compatible device authorization challenge.
///
/// # Arguments
/// client supplies transport; domain selects the HTTPS host; user_agent and client_id identify the caller.
/// # Returns
/// The device/user codes, verification URL and polling interval.
/// # Errors
/// Returns transport, rejected-request or decoding errors.
/// # Examples
/// ```no_run
/// # async fn example(client: &reqwest::Client, issuer: &str) -> anyhow::Result<()> {
/// let device = codetether_device_auth::request_device_code(client, issuer, "example/1", "client-id").await?;
/// # Ok(()) }
/// ```
pub async fn request_device_code(
    client: &Client,
    domain: &str,
    user_agent: &str,
    client_id: &str,
) -> Result<DeviceCodeResponse> {
    let url = format!("https://{domain}/login/device/code");
    let response = client
        .post(&url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("User-Agent", user_agent)
        .json(&json!({
            "client_id": client_id,
            "scope": "read:user",
        }))
        .send()
        .await
        .with_context(|| format!("Failed to reach device authorization endpoint: {url}"))?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!(
            "Failed to initiate device authorization ({}): {}",
            status,
            truncate_body(&body)
        );
    }

    let mut device: DeviceCodeResponse = response
        .json()
        .await
        .context("Failed to parse device authorization response")?;
    if device.interval.unwrap_or(0) == 0 {
        device.interval = Some(5);
    }
    Ok(device)
}
