//! Fetch and cache the account Codex model catalog.
use crate::endpoints::{CHATGPT_CODEX_API_URL, CHATGPT_CODEX_CLIENT_VERSION};
use anyhow::{Context, Result};
use codetether_provider_core::codex::model_discovery;
/// Fetch, parse, log, and store a fresh account model catalog.
///
/// # Arguments
/// * `client` - The provider HTTP client.
/// * `access_token` - Bearer token acquired by the application.
/// * `account_id` - ChatGPT workspace identifier.
/// # Returns
/// Discovered models after updating the shared core cache.
/// # Errors
/// Returns send/read, non-success status, or catalog parse errors.
pub async fn discover_chatgpt_models(
    client: &reqwest::Client,
    access_token: &str,
    account_id: &str,
) -> Result<Vec<model_discovery::DiscoveredModel>> {
    let url =
        format!("{CHATGPT_CODEX_API_URL}/models?client_version={CHATGPT_CODEX_CLIENT_VERSION}");
    let response = client
        .get(url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("chatgpt-account-id", account_id)
        .header("version", CHATGPT_CODEX_CLIENT_VERSION)
        .header("originator", "codex_cli_rs")
        .send()
        .await
        .context("Failed to query ChatGPT Codex /models")?;
    let status = response.status();
    let body = response.bytes().await?;
    if !status.is_success() {
        anyhow::bail!("ChatGPT Codex /models returned HTTP {status}");
    }
    let models = model_discovery::parse_models(&body)?;
    let slugs: Vec<String> = models
        .iter()
        .map(|m| format!("{}({})", m.slug, m.visibility.as_deref().unwrap_or("-")))
        .collect();
    tracing::info!(
        provider = "openai-codex",
        count = models.len(),
        models = %slugs.join(","),
        "Discovered Codex models"
    );
    model_discovery::store(models.clone());
    Ok(models)
}
