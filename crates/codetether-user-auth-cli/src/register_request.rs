//! HTTP account registration and response validation.
use crate::register_response::RegisterResponse;
use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::json;

pub(super) async fn send(
    client: &Client,
    server_url: &str,
    email: &str,
    password: &str,
    first_name: &Option<String>,
    last_name: &Option<String>,
    referral_source: &Option<String>,
    application_version: &str,
) -> Result<RegisterResponse> {
    let user_agent = format!("codetether-agent/{}", application_version);

    let resp = client
        .post(format!("{}/v1/users/register", server_url))
        .header("User-Agent", &user_agent)
        .header("Content-Type", "application/json")
        .json(&json!({
            "email": email,
            "password": password,
            "first_name": first_name,
            "last_name": last_name,
            "referral_source": referral_source,
        }))
        .send()
        .await
        .context("Failed to connect to CodeTether server")?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body: serde_json::Value = resp.json().await.unwrap_or_default();
        let detail = body
            .get("detail")
            .and_then(|v| v.as_str())
            .unwrap_or("Registration failed");
        anyhow::bail!("Registration failed ({}): {}", status, detail);
    }

    let reg: RegisterResponse = resp
        .json()
        .await
        .context("Failed to parse registration response")?;

    Ok(reg)
}
