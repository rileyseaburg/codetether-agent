//! Buffered completion for the application protocol conventions.
use crate::CompletionResponse;
use anyhow::Result;
use reqwest::Client;
use serde_json::Value;
/// Execute an application-format buffered request.
///
/// # Arguments
/// * client - Shared HTTP client.
/// * api_key - Bearer credential.
/// * base_url - Completion API endpoint prefix.
/// * body - Already-converted application request payload.
///
/// # Returns
/// The decoded assistant response and token accounting.
///
/// # Errors
/// Returns transport, non-success HTTP, malformed JSON, or missing-choice errors.
///
/// # Examples
///
///     # async fn example() -> anyhow::Result<()> {
///     let client = reqwest::Client::new();
///     let body = serde_json::json!({"model":"glm-5","messages":[]});
///     let response = codetether_provider_zai::completion::complete_application(
///         &client, "", "http://localhost:1234", &body,
///     ).await?;
///     let _ = response;
///     # Ok(()) }
pub async fn complete_application(
    client: &Client,
    api_key: &str,
    base_url: &str,
    body: &Value,
) -> Result<CompletionResponse> {
    let (text, status) = super::send::send(client, api_key, base_url, body).await?;
    if !status.is_success() {
        tracing::debug!(status = %status, body = %text, "Z.AI error response");
        anyhow::bail!("{}", crate::error_wire::full_message(status, &text));
    }

    super::parse::parse(&text)
}
