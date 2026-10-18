//! Registration flow for new users (onboarding).

use crate::{DEFAULT_SERVER, prompt_line};
use anyhow::{Context, Result};
use codetether_user_auth::{
    SavedCredentials, load_saved_credentials, login_with_password, write_saved_credentials,
};
use codetether_user_auth_cli::rpassword_prompt;
use reqwest::Client;
use serde_json::json;

/// Run interactive account registration followed by login.
/// # Arguments
/// * `version` - Calling application's version sent during login.
/// # Returns
/// Credentials reloaded from the shared local cache after successful login.
/// # Errors
/// Returns validation, prompt, network, authentication or persistence errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let _credentials = codetether_onboarding::register_and_login("1.0").await?;
/// # Ok(()) }
/// ```
pub async fn register_and_login(version: &str) -> Result<Option<SavedCredentials>> {
    let email = prompt_line("Email: ")?;
    if email.is_empty() {
        anyhow::bail!("Email is required");
    }
    let password = rpassword_prompt("Password (min 8 chars): ")?;
    if password.len() < 8 {
        anyhow::bail!("Password must be at least 8 characters");
    }
    let confirm = rpassword_prompt("Confirm password: ")?;
    if password != confirm {
        anyhow::bail!("Passwords do not match");
    }

    println!("Creating your account...");
    let client = Client::new();
    let resp = client
        .post(format!("{DEFAULT_SERVER}/v1/users/register"))
        .header("Content-Type", "application/json")
        .json(&json!({ "email": email, "password": password }))
        .send()
        .await
        .context("Failed to connect to CodeTether")?;

    if !resp.status().is_success() {
        let body: serde_json::Value = resp.json().await.unwrap_or_default();
        let detail = body
            .get("detail")
            .and_then(|v| v.as_str())
            .unwrap_or("Registration failed");
        anyhow::bail!("{detail}");
    }

    println!("Account created! Logging in...");
    let login = login_with_password(&client, DEFAULT_SERVER, &email, &password, version).await?;
    write_saved_credentials(DEFAULT_SERVER, &email, &login)?;
    println!("Logged in as {email}.\n");
    load_saved_credentials()
        .map(Some)
        .ok_or_else(|| anyhow::anyhow!("Failed to load credentials"))
}
