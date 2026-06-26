//! OIDC `refresh_token` grant — mint a new SSO access token without a browser.
//!
//! While the IdP/SSO refresh token is still valid, this exchanges it for a
//! fresh access token via `CreateToken` (grant type `refresh_token`), so no
//! device-code prompt or browser interaction is required.

use anyhow::{Context, Result, bail};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;

/// Stored OIDC client + refresh material needed for a silent refresh.
pub(super) struct RefreshInputs<'a> {
    pub region: &'a str,
    pub client_id: &'a str,
    pub client_secret: &'a str,
    pub refresh_token: &'a str,
}

/// Subset of the `CreateToken` refresh response we consume.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct RefreshedToken {
    pub access_token: String,
}

/// Exchange a refresh token for a fresh SSO access token (no browser).
pub(super) async fn refresh_access_token(
    http: &Client,
    inputs: RefreshInputs<'_>,
) -> Result<RefreshedToken> {
    let url = format!("https://oidc.{}.amazonaws.com/token", inputs.region);
    let resp = http
        .post(url)
        .json(&json!({
            "clientId": inputs.client_id,
            "clientSecret": inputs.client_secret,
            "refreshToken": inputs.refresh_token,
            "grantType": "refresh_token",
        }))
        .send()
        .await
        .context("refresh CreateToken request failed")?;
    if !resp.status().is_success() {
        bail!(
            "refresh CreateToken HTTP {}: {}",
            resp.status(),
            resp.text().await.unwrap_or_default()
        );
    }
    resp.json().await.context("refresh CreateToken: bad JSON")
}
