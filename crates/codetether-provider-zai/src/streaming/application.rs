//! Streaming with application request and usage conventions.
use crate::StreamChunk;
use anyhow::Result;
use futures::{StreamExt, stream::BoxStream};
use reqwest::Client;
use serde_json::Value;
/// Execute an application-format stream with total prompt-token accounting.
///
/// # Arguments
/// * client - Shared HTTP client.
/// * api_key - Bearer credential.
/// * base_url - Completion endpoint prefix.
/// * body - Encoded streaming request.
///
/// # Returns
/// Text, reasoning, tool-call, error, and terminal usage chunks.
///
/// # Errors
/// Returns transport errors before receiving a response; HTTP failures become
/// error and terminal chunks.
///
/// # Examples
///
///     # async fn example() -> anyhow::Result<()> {
///     let client = reqwest::Client::new();
///     let body = serde_json::json!({"model":"glm-5","messages":[],"stream":true});
///     let stream = codetether_provider_zai::streaming::complete_application(
///         &client, "", "http://localhost:1234", &body,
///     ).await?;
///     let _ = stream;
///     # Ok(()) }
pub async fn complete_application(
    client: &Client,
    api_key: &str,
    base_url: &str,
    body: &Value,
) -> Result<BoxStream<'static, StreamChunk>> {
    let response = super::send::send(client, api_key, base_url, body).await?;
    if !response.status().is_success() {
        return Ok(futures::stream::iter(super::error::chunks(response).await).boxed());
    }
    Ok(super::decode::decode_application(response))
}
