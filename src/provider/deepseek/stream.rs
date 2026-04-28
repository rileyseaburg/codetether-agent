//! Streaming completion for DeepSeek (falls back to non-streaming).

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
    let text = response
        .message
        .content
        .iter()
        .filter_map(|p| match p {
            ContentPart::Text { text } => Some(text.clone()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("");
    Ok(Box::pin(futures::stream::once(async move {
        StreamChunk::Text(text)
    })))
}
