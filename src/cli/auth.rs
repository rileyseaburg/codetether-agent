//! Provider authentication commands.

use super::{AuthArgs, AuthCommand, CopilotAuthArgs};
use crate::provider::copilot::normalize_enterprise_domain;
use crate::secrets::{self, ProviderSecrets};
use anyhow::{Context, Result};
use reqwest::Client;
use serde::Deserialize;
use serde_json::json;
use std::collections::HashMap;
use tokio::time::{Duration, sleep};

const DEFAULT_GITHUB_DOMAIN: &str = "github.com";
const OAUTH_POLLING_SAFETY_MARGIN_MS: u64 = 3000;

#[derive(Debug, Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    #[serde(default)]
    verification_uri_complete: Option<String>,
    #[serde(default)]
    interval: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct AccessTokenResponse {
    #[serde(default)]
    access_token: Option<String>,
    #[serde(default)]
    error: Option<String>,
    #[serde(default)]
    error_description: Option<String>,
    #[serde(default)]
    interval: Option<u64>,
}

pub async fn execute(args: AuthArgs) -> Result<()> {
    match args.command {
        AuthCommand::Copilot(copilot_args) => authenticate_copilot(copilot_args).await,
    }
}

async fn authenticate_copilot(args: CopilotAuthArgs) -> Result<()> {
    if secrets::secrets_manager().is_none() {
        anyhow::bail!(
            "HashiCorp Vault is not configured. Set VAULT_ADDR and VAULT_TOKEN before running `codetether auth copilot`."
        );
    }

    let (provider_id, domain, enterprise_domain) = match args.enterprise_url {
        Some(raw) => {
            let domain = normalize_enterprise_domain(&raw);
            if domain.is_empty() {
                anyhow::bail!("--enterprise-url cannot be empty");
            }
            ("github-copilot-enterprise", domain.clone(), Some(domain))
        }
        None => ("github-copilot", DEFAULT_GITHUB_DOMAIN.to_string(), None),
    };

    let client = Client::new();
    let client_id = resolve_client_id(args.client_id)?;
    let user_agent = format!("codetether-agent/{}", env!("CARGO_PKG_VERSION"));
    let device = request_device_code(&client, &domain, &user_agent, &client_id).await?;

    println!("GitHub Copilot device authentication");
    println!(
        "Open this URL: {}",
        device
            .verification_uri_complete
            .as_deref()
            .unwrap_or(&device.verification_uri)
    );
    println!("Enter code: {}", device.user_code);
    println!("Waiting for authorization...");

    let token = poll_for_access_token(&client, &domain, &user_agent, &client_id, &device).await?;

    let mut extra = HashMap::new();
    if let Some(enterprise_url) = enterprise_domain {
        extra.insert(
            "enterpriseUrl".to_string(),
            serde_json::Value::String(enterprise_url),
        );
    }

    let provider_secrets = ProviderSecrets {
        api_key: Some(token),
        base_url: None,
        organization: None,
        headers: None,
        extra,
    };

    secrets::set_provider_secrets(provider_id, &provider_secrets)
        .await
        .with_context(|| format!("Failed to store {} auth token in Vault", provider_id))?;

    println!("Saved {} credentials to HashiCorp Vault.", provider_id);
    Ok(())
}

async fn request_device_code(
    client: &Client,
    domain: &str,
    user_agent: &str,
    client_id: &str,
) -> Result<DeviceCodeResponse> {
    let url = format!("https://{domain}/login/device/code");
    let response = client
        .post(&url)
        .header("Accept", "application/json")
        .header("Content-Type", "application/json")
        .header("User-Agent", user_agent)
        .json(&json!({
            "client_id": client_id,
            "scope": "read:user",
        }))
        .send()
        .await
        .with_context(|| format!("Failed to reach device authorization endpoint: {url}"))?;

    let status = response.status();
    if !status.is_success() {
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!(
            "Failed to initiate device authorization ({}): {}",
            status,
            truncate_body(&body)
        );
    }

    let mut device: DeviceCodeResponse = response
        .json()
        .await
        .context("Failed to parse device authorization response")?;
    if device.interval.unwrap_or(0) == 0 {
        device.interval = Some(5);
    }
    Ok(device)
}

async fn poll_for_access_token(
    client: &Client,
    domain: &str,
    user_agent: &str,
    client_id: &str,
    device: &DeviceCodeResponse,
) -> Result<String> {
    let url = format!("https://{domain}/login/oauth/access_token");
    let mut interval_secs = device.interval.unwrap_or(5).max(1);

    loop {
        let response = client
            .post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("User-Agent", user_agent)
            .json(&json!({
                "client_id": client_id,
                "device_code": device.device_code,
                "grant_type": "urn:ietf:params:oauth:grant-type:device_code",
            }))
            .send()
            .await
            .with_context(|| format!("Failed to poll token endpoint: {url}"))?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!(
                "Failed to exchange device code for access token ({}): {}",
                status,
                truncate_body(&body)
            );
        }

        let payload: AccessTokenResponse = response
            .json()
            .await
            .context("Failed to parse OAuth token response")?;

        if let Some(token) = payload.access_token {
            if !token.trim().is_empty() {
                return Ok(token);
            }
        }

        match payload.error.as_deref() {
            Some("authorization_pending") => sleep_with_margin(interval_secs).await,
            Some("slow_down") => {
                interval_secs = payload
                    .interval
                    .filter(|value| *value > 0)
                    .unwrap_or(interval_secs + 5);
                sleep_with_margin(interval_secs).await;
            }
            Some(error) => {
                let description = payload
                    .error_description
                    .unwrap_or_else(|| "No error description provided".to_string());
                anyhow::bail!("Copilot OAuth failed: {} ({})", error, description);
            }
            None => sleep_with_margin(interval_secs).await,
        }
    }
}

fn resolve_client_id(client_id: Option<String>) -> Result<String> {
    let id = client_id
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "GitHub OAuth client ID is required. Pass `--client-id <id>` or set `CODETETHER_COPILOT_OAUTH_CLIENT_ID`."
            )
        })?;

    Ok(id)
}

async fn sleep_with_margin(interval_secs: u64) {
    sleep(Duration::from_millis(
        interval_secs.saturating_mul(1000) + OAUTH_POLLING_SAFETY_MARGIN_MS,
    ))
    .await;
}

fn truncate_body(body: &str) -> String {
    const MAX_LEN: usize = 300;
    if body.len() <= MAX_LEN {
        body.to_string()
    } else {
        format!("{}...", &body[..MAX_LEN])
    }
}
