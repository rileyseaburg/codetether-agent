//! `StartDeviceAuthorization` call against SSO-OIDC.
//!
//! Unauthenticated JSON POST to `oidc.<region>.amazonaws.com` that begins
//! the device-code grant and returns the user code + verification URL.

use anyhow::{Context, Result, bail};
use reqwest::Client;
use serde_json::json;

use super::oidc_types::{DeviceAuthorization, RegisteredClient};

/// Begin device authorization for `start_url`, returning the user code + URL.
pub(super) async fn start_device_authorization(
    http: &Client,
    region: &str,
    client: &RegisteredClient,
    start_url: &str,
) -> Result<DeviceAuthorization> {
    let url = format!("https://oidc.{region}.amazonaws.com/device_authorization");
    let resp = http
        .post(url)
        .json(&json!({
            "clientId": client.client_id,
            "clientSecret": client.client_secret,
            "startUrl": start_url,
        }))
        .send()
        .await
        .context("StartDeviceAuthorization request failed")?;
    if !resp.status().is_success() {
        bail!(
            "StartDeviceAuthorization HTTP {}: {}",
            resp.status(),
            resp.text().await.unwrap_or_default()
        );
    }
    resp.json()
        .await
        .context("StartDeviceAuthorization: bad JSON")
}
