//! `CreateToken` polling loop for the SSO-OIDC device-code flow.
//!
//! After the user approves the device code in their browser, this polls
//! `CreateToken` until it returns an access token (or a terminal error),
//! honouring the `authorization_pending` / `slow_down` back-off codes.

use std::time::Duration;

use anyhow::{Context, Result, bail};
use reqwest::Client;
use serde_json::json;

use super::oidc_types::{DeviceAuthorization, DeviceToken, RegisteredClient};

/// Poll `CreateToken` until the user authorizes or the request fails.
pub(super) async fn poll_for_token(
    http: &Client,
    region: &str,
    client: &RegisteredClient,
    auth: &DeviceAuthorization,
) -> Result<DeviceToken> {
    let url = format!("https://oidc.{region}.amazonaws.com/token");
    let mut interval = auth.interval.unwrap_or(5).max(1);
    loop {
        let resp = http
            .post(&url)
            .json(&json!({
                "clientId": client.client_id,
                "clientSecret": client.client_secret,
                "deviceCode": auth.device_code,
                "grantType": "urn:ietf:params:oauth:grant-type:device_code",
            }))
            .send()
            .await
            .context("CreateToken request failed")?;
        if resp.status().is_success() {
            return resp.json().await.context("CreateToken: bad JSON");
        }
        let body = resp.text().await.unwrap_or_default();
        match error_code(&body).as_str() {
            "authorization_pending" => {}
            "slow_down" => interval += 5,
            other => bail!("CreateToken failed ({other}): {body}"),
        }
        tokio::time::sleep(Duration::from_secs(interval)).await;
    }
}

/// Extract the OAuth `error` field from a CreateToken error body.
fn error_code(body: &str) -> String {
    serde_json::from_str::<serde_json::Value>(body)
        .ok()
        .and_then(|v| v.get("error").and_then(|e| e.as_str()).map(str::to_string))
        .unwrap_or_else(|| "unknown".to_string())
}
