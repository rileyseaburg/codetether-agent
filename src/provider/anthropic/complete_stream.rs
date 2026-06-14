//! Streaming completion execution for Anthropic-compatible providers.
//!
//! Sends a Messages API request with `stream: true` and converts the SSE
//! response into provider-neutral [`StreamChunk`] values.

use anyhow::Result;
use futures::StreamExt;
use futures::stream::BoxStream;
use serde_json::Value;

use crate::provider::StreamChunk;

use super::sse_stream::SseChunkStream;

/// Send a streaming request and return a boxed stream of chunks.
///
/// The request body must already include `"stream": true`.
pub(crate) async fn start(
    client: &reqwest::Client,
    base_url: &str,
    api_key: &str,
    body: &Value,
) -> Result<BoxStream<'static, StreamChunk>> {
    let url = format!("{}/v1/messages", base_url.trim_end_matches('/'));
    let resp = client
        .post(url)
        .header("x-api-key", api_key)
        .header("anthropic-version", super::constants::ANTHROPIC_VERSION)
        .header("content-type", "application/json")
        .json(body)
        .send()
        .await?;
    let status = resp.status();
    if !status.is_success() {
        let text = resp.text().await.unwrap_or_default();
        return Err(super::complete_error::map(status.as_u16(), &text));
    }
    Ok(SseChunkStream::new(resp).boxed())
}
