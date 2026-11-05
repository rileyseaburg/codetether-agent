//! Register a public client for one exact loopback redirect.
use super::{validation::valid_secret, wire::Registration};
use crate::discovery::{self, REGISTER};
use anyhow::{Result, anyhow, ensure};
use reqwest::Client;

pub(super) async fn register(client: &Client, redirect: &str, scope: &str) -> Result<String> {
    let response = client.post(REGISTER).json(&serde_json::json!({
        "client_name": "CodeTether", "redirect_uris": [redirect],
        "token_endpoint_auth_method": "none", "grant_types": ["authorization_code", "refresh_token"],
        "response_types": ["code"], "scope": scope,
    })).send().await.map_err(|_| anyhow!("Monarch client registration failed"))?;
    let registration: Registration = discovery::json(response).await?;
    ensure!(
        valid_secret(&registration.client_id)
            && registration.token_endpoint_auth_method == "none"
            && registration.redirect_uris == [redirect],
        "Invalid Monarch client registration"
    );
    Ok(registration.client_id)
}
