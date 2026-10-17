//! Submit the authenticated direct OpenRouter HTTP request.
use anyhow::{Context, Result};
pub(super) async fn apply_with_direct_openrouter(prompt: &str, base_url: &str) -> Result<String> {
    let api_key = std::env::var("OPENROUTER_API_KEY").context(
        "OPENROUTER_API_KEY is required when CODETETHER_OPENROUTER_BASE_URL is set for direct Morph backend calls",
    )?;

    let body = super::direct_request::body(prompt);
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("{base_url}/chat/completions"))
        .header("Authorization", format!("Bearer {api_key}"))
        .header("Content-Type", "application/json")
        .header("HTTP-Referer", "https://codetether.run")
        .header("X-Title", "CodeTether Agent")
        .json(&body)
        .send()
        .await
        .context("Failed to send Morph request")?;

    let status = resp.status();
    let text = resp.text().await.context("Failed to read Morph response")?;

    super::direct_response::parse(status, text)
}
