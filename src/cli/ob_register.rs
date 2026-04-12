//! Registration flow for new users (onboarding).

use crate::cli::auth::{
    SavedCredentials, load_saved_credentials, login_with_password, rpassword_prompt,
    write_saved_credentials,
};
use crate::cli::onboard::{DEFAULT_SERVER, prompt_line};
use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::json;

pub async fn register_and_login() -> Result<Option<SavedCredentials>> {
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
    let login = login_with_password(&client, DEFAULT_SERVER, &email, &password).await?;
    write_saved_credentials(DEFAULT_SERVER, &email, &login)?;
    println!("Logged in as {email}.\n");
    load_saved_credentials()
        .map(Some)
        .ok_or_else(|| anyhow::anyhow!("Failed to load credentials"))
}
