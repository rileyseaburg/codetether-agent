//! Device authorization codex_poll responsibilities.
use crate::{
    CODEX_DEVICE_AUTH_TIMEOUT_SECS, CodexDeviceCodeResponse, CodexDeviceCodeTokenResponse,
    timing::sleep_with_margin,
};
use anyhow::{Context, Result};
use reqwest::Client;
use tokio::time::{Duration, Instant};

/// Poll a Codex device challenge until authorization or failure.
///
/// # Arguments
/// client supplies transport; issuer and user_agent identify the endpoint/caller; device is the original challenge.
/// # Returns
/// An authorization code and its verifier.
/// # Errors
/// Returns transport, decoding, timeout or authorization errors.
/// # Examples
/// ```no_run
/// # async fn example(client: &reqwest::Client, issuer: &str) -> anyhow::Result<()> {
/// let device = codetether_device_auth::request_codex_device_code(client, issuer, "example/1", "client-id").await?;
/// let code = codetether_device_auth::poll_for_codex_authorization_code(client, issuer, "example/1", &device).await?;
/// # Ok(()) }
/// ```
pub async fn poll_for_codex_authorization_code(
    client: &Client,
    issuer: &str,
    user_agent: &str,
    device: &CodexDeviceCodeResponse,
) -> Result<CodexDeviceCodeTokenResponse> {
    let url = format!("{issuer}/api/accounts/deviceauth/token");
    let interval_secs = device.interval.max(1);
    let timeout = Duration::from_secs(CODEX_DEVICE_AUTH_TIMEOUT_SECS);
    let start = Instant::now();

    loop {
        let response = super::codex_poll_request::post(client, &url, user_agent, device).await?;
        let status = response.status();
        if status.is_success() {
            return response
                .json()
                .await
                .context("Failed to parse Codex device authorization response");
        }

        let body = response.text().await.unwrap_or_default();
        if status == reqwest::StatusCode::FORBIDDEN || status == reqwest::StatusCode::NOT_FOUND {
            if start.elapsed() >= timeout {
                anyhow::bail!(
                    "Device authorization timed out after {} seconds",
                    CODEX_DEVICE_AUTH_TIMEOUT_SECS
                );
            }
            sleep_with_margin(interval_secs).await;
            continue;
        }

        return super::codex_failure::reject(status, &body);
    }
}
