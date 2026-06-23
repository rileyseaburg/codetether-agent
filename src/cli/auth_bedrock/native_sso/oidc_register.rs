//! `RegisterClient` call against SSO-OIDC.
//!
//! Unauthenticated JSON POST to `oidc.<region>.amazonaws.com` that mints a
//! public OIDC client id/secret used to start a device-code login.

use anyhow::{Context, Result, bail};
use reqwest::Client;
use serde_json::json;

use super::oidc_types::RegisteredClient;

pub(super) use super::oidc_device_auth::start_device_authorization;

/// Register a public OIDC client for device-code use.
pub(super) async fn register_client(http: &Client, region: &str) -> Result<RegisteredClient> {
    let url = format!("https://oidc.{region}.amazonaws.com/client/register");
    let resp = http
        .post(url)
        .json(&json!({
            "clientName": "codetether-bedrock-auth",
            "clientType": "public",
            "scopes": ["sso:account:access"],
        }))
        .send()
        .await
        .context("RegisterClient request failed")?;
    if !resp.status().is_success() {
        bail!(
            "RegisterClient HTTP {}: {}",
            resp.status(),
            resp.text().await.unwrap_or_default()
        );
    }
    resp.json().await.context("RegisterClient: bad JSON")
}
