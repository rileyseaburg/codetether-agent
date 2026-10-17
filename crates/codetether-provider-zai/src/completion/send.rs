//! Buffered HTTP sending with the existing retry policy.
use anyhow::{Context, Result};
use reqwest::{Client, StatusCode};
use serde_json::Value;
pub(super) async fn send(
    client: &Client,
    api_key: &str,
    base_url: &str,
    body: &Value,
) -> Result<(String, StatusCode)> {
    codetether_provider_core::retry::send_with_retry(|| async {
        let resp = client
            .post(format!("{}/chat/completions", base_url))
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .context("Failed to send request to Z.AI")?;
        let status = resp.status();
        let text = resp.text().await.context("Failed to read Z.AI response")?;
        Ok((text, status))
    })
    .await
}
