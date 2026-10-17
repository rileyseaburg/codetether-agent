//! Application streaming request transport.
use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::Value;
pub(super) async fn send(
    client: &Client,
    api_key: &str,
    base_url: &str,
    body: &Value,
) -> Result<reqwest::Response> {
    codetether_provider_core::retry::send_response_with_retry(|| async {
        client
            .post(format!("{}/chat/completions", base_url))
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send streaming request to Z.AI")
    })
    .await
}
