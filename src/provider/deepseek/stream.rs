//! Streaming completion for DeepSeek (falls back to non-streaming).
//!
//! Until native SSE parsing is implemented, we fall back to a
//! single non-streaming call and emit the full response as chunks.

use crate::provider::{CompletionRequest, ContentPart, StreamChunk};
use anyhow::Result;
use futures::stream::BoxStream;

use super::DeepSeekProvider;
use super::complete;

pub(crate) async fn exec(
    p: &DeepSeekProvider,
    req: CompletionRequest,
) -> Result<BoxStream<'static, StreamChunk>> {
    tracing::debug!(provider = "deepseek", model = %req.model, "Streaming (falling back to non-streaming)");
    let response = complete::exec(p, req).await?;

    let mut chunks: Vec<StreamChunk> = Vec::new();
    for part in &response.message.content {
        match part {
            ContentPart::Text { text } => {
                chunks.push(StreamChunk::Text(text.clone()));
            }
            ContentPart::ToolCall { id, name, arguments, .. } => {
                chunks.push(StreamChunk::ToolCallStart {
                    id: id.clone(),
                    name: name.clone(),
                });
                chunks.push(StreamChunk::ToolCallDelta {
                    id: id.clone(),
                    arguments_delta: arguments.clone(),
                });
                chunks.push(StreamChunk::ToolCallEnd { id: id.clone() });
            }
            _ => {}
        }
    }

    if chunks.is_empty() {
        chunks.push(StreamChunk::Text(String::new()));
    }

    Ok(Box::pin(futures::stream::iter(chunks)))
}
