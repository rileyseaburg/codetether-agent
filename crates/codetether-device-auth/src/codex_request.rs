//! Device authorization codex_request responsibilities.
use crate::{CodexDeviceCodeResponse, error_text::truncate_body};
use anyhow::{Context, Result};
use reqwest::Client;

/// Request the Codex device authorization challenge.
///
/// # Arguments
/// client supplies transport; issuer is the base URL; user_agent and client_id identify the caller.
/// # Returns
/// The device authorization ID, user code and polling interval.
/// # Errors
/// Returns transport, rejected-request or decoding errors.
/// # Examples
/// ```no_run
/// # async fn example(client: &reqwest::Client, issuer: &str) -> anyhow::Result<()> {
/// let device = codetether_device_auth::request_codex_device_code(client, issuer, "example/1", "client-id").await?;
/// # Ok(()) }
/// ```
pub async fn request_codex_device_code(
    client: &Client,
    issuer: &str,
    user_agent: &str,
    client_id: &str,
) -> Result<CodexDeviceCodeResponse> {
    let response = super::codex_start_request::post(client, issuer, user_agent, client_id).await?;
    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        if status == reqwest::StatusCode::NOT_FOUND {
            anyhow::bail!(
                "Device code login is not enabled for this Codex server. Use browser OAuth flow instead."
            );
        }
        anyhow::bail!(
            "Failed to initiate Codex device authorization ({}): {}",
            status,
            truncate_body(&body)
        );
    }

    let mut device: CodexDeviceCodeResponse = response
        .json()
        .await
        .context("Failed to parse Codex device authorization response")?;
    if device.interval == 0 {
        device.interval = 5;
    }
    Ok(device)
}
