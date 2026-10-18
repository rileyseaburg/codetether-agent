//! Device authorization github_poll responsibilities.
use crate::{DeviceCodeResponse, github_types::AccessTokenResponse, timing::sleep_with_margin};
use anyhow::Result;
use reqwest::Client;

/// Poll a GitHub-compatible challenge for an access token.
///
/// # Arguments
/// client supplies transport; domain, user_agent and client_id identify the request; device is the challenge.
/// # Returns
/// The nonempty access token after authorization.
/// # Errors
/// Returns transport, decoding or OAuth response errors.
/// # Examples
/// ```no_run
/// # async fn example(client: &reqwest::Client, issuer: &str) -> anyhow::Result<()> {
/// let device = codetether_device_auth::request_device_code(client, issuer, "example/1", "client-id").await?;
/// let token = codetether_device_auth::poll_for_access_token(client, issuer, "example/1", "client-id", &device).await?;
/// # Ok(()) }
/// ```
pub async fn poll_for_access_token(
    client: &Client,
    domain: &str,
    user_agent: &str,
    client_id: &str,
    device: &DeviceCodeResponse,
) -> Result<String> {
    let url = format!("https://{domain}/login/oauth/access_token");
    let mut interval_secs = device.interval.unwrap_or(5).max(1);

    loop {
        let payload =
            super::github_poll_request::payload(client, &url, user_agent, client_id, device)
                .await?;
        if let Some(token) = payload.access_token
            && !token.trim().is_empty()
        {
            return Ok(token);
        }

        match payload.error.as_deref() {
            Some("authorization_pending") => sleep_with_margin(interval_secs).await,
            Some("slow_down") => {
                interval_secs = payload
                    .interval
                    .filter(|value| *value > 0)
                    .unwrap_or(interval_secs + 5);
                sleep_with_margin(interval_secs).await;
            }
            Some(error) => {
                let description = payload
                    .error_description
                    .unwrap_or_else(|| "No error description provided".to_string());
                anyhow::bail!("Copilot OAuth failed: {} ({})", error, description);
            }
            None => sleep_with_margin(interval_secs).await,
        }
    }
}
