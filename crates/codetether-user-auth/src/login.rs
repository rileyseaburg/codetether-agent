//! Shared user authentication login operations.

use crate::LoginResponsePayload;
use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::json;

/// Authenticate a user against the selected CodeTether server.
///
/// # Arguments
/// * `client` - HTTP client used for the request.
/// * `server_url` - Server base URL.
/// * `email` - Account email address.
/// * `password` - Password supplied by the user.
/// * `application_version` - Caller version embedded in the User-Agent.
/// # Returns
/// The server's credential and user response.
/// # Errors
/// Returns request, rejected-login or response-decoding errors.
/// # Examples
/// ```no_run
/// # async fn example(client: &reqwest::Client, password: &str) -> anyhow::Result<()> {
/// let reply = codetether_user_auth::login_with_password(
///     client, "https://example.invalid", "user@example.invalid", password, "1.0",
/// ).await?;
/// # Ok(()) }
/// ```
pub async fn login_with_password(
    client: &Client,
    server_url: &str,
    email: &str,
    password: &str,
    application_version: &str,
) -> Result<LoginResponsePayload> {
    let user_agent = format!("codetether-agent/{}", application_version);

    let resp = client
        .post(format!("{}/v1/users/login", server_url))
        .header("User-Agent", &user_agent)
        .header("Content-Type", "application/json")
        .json(&json!({
            "email": email,
            "password": password,
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
            .unwrap_or("Authentication failed");
        anyhow::bail!("Login failed ({}): {}", status, detail);
    }

    let login: LoginResponsePayload = resp
        .json()
        .await
        .context("Failed to parse login response")?;

    Ok(login)
}
