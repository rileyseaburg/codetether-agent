//! Consume protocol chunks into one buffered completion.
use crate::{Chunks, StreamChunk};
use anyhow::Result;
use codetether_codex_protocol::CompletionCollector;
use codetether_provider_core::CompletionResponse;
use futures::StreamExt;

/// Consume chunks through Done and construct a buffered response.
///
/// # Arguments
/// * `stream` - Decoded Codex chunks from either transport.
/// # Returns
/// The aggregated text, reasoning, tools, and usage.
/// # Errors
/// Returns a stream error or an availability error when Done is missing.
pub async fn collect_stream_completion(mut stream: Chunks) -> Result<CompletionResponse> {
    let mut collector = CompletionCollector::default();
    while let Some(chunk) = stream.next().await {
        let done = matches!(chunk, StreamChunk::Done { .. });
        collector.accept(chunk)?;
        if done {
            break;
        }
    }
    collector.finish()
}
