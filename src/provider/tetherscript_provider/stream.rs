//! Streaming fallback — delegates to non-streaming [`complete`].

use anyhow::Result;
use futures::stream::BoxStream;

use super::runner::TetherScriptProvider;
use crate::provider::{CompletionRequest, StreamChunk};

/// Complete via non-streaming, emit as a single-chunk stream.
pub async fn complete_stream(
    provider: &TetherScriptProvider,
    req: CompletionRequest,
) -> Result<BoxStream<'static, StreamChunk>> {
    let resp = super::complete::complete(provider, req).await?;
    let text = resp
        .message
        .content
        .iter()
        .filter_map(|p| match p {
            crate::provider::ContentPart::Text { text } => Some(text.clone()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("");
    let chunks = vec![
        StreamChunk::Text(text),
        StreamChunk::Done {
            usage: Some(resp.usage),
        },
    ];
    Ok(Box::pin(futures::stream::iter(chunks)))
}
