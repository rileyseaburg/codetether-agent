//! HTTP transport for Anthropic completion requests.

use anyhow::{Context, Result};

use super::AnthropicProvider;
use super::constants::ANTHROPIC_VERSION;
use super::response::AnthropicResponse;

/// Send an Anthropic Messages API request and parse the response envelope.
pub(crate) async fn send(
    p: &AnthropicProvider,
    body: &serde_json::Value,
) -> Result<AnthropicResponse> {
    let response = p
        .client
        .post(format!("{}/v1/messages", p.base_url.trim_end_matches('/')))
        .header("x-api-key", &p.api_key)
        .header("anthropic-version", ANTHROPIC_VERSION)
        .header("content-type", "application/json")
        .json(body)
        .send()
        .await
        .context("Failed to send request to Anthropic")?;
    let status = response.status();
    let text = response
        .text()
        .await
        .context("Failed to read Anthropic response")?;
    if !status.is_success() {
        return Err(super::complete_error::map(status.as_u16(), &text));
    }
    serde_json::from_str(&text).context(format!(
        "Failed to parse Anthropic response: {}",
        super::complete::safe_char_prefix(&text, 200)
    ))
}
