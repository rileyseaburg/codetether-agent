//! Start-request transport for Codex device authorization.
use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::json;

pub(super) async fn post(
    client: &Client,
    issuer: &str,
    user_agent: &str,
    client_id: &str,
) -> Result<reqwest::Response> {
    let url = format!("{issuer}/api/accounts/deviceauth/usercode");
    let response = client
        .post(&url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("User-Agent", user_agent)
        .json(&json!({
            "client_id": client_id,
        }))
        .send()
        .await
        .with_context(|| format!("Failed to reach device authorization endpoint: {url}"))?;

    Ok(response)
}
