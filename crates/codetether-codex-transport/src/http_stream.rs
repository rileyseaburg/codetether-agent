//! Idle-bounded SSE consumption and terminal error handling.
use crate::{Chunks, StreamChunk};
use codetether_codex_protocol::ResponsesSseParser;

/// Consume an HTTP response with a 300-second idle deadline per SSE event.
///
/// # Arguments
/// * `response` - A successful, unconsumed streaming HTTP response.
/// # Returns
/// Chunks ending at Done or the original transport/idle failure.
/// # Examples
/// ```no_run
/// # async fn example(response: reqwest::Response) {
/// let stream = codetether_codex_transport::drive_responses_http_stream(response);
/// # }
/// ```
pub fn drive_responses_http_stream(response: reqwest::Response) -> Chunks {
    Box::pin(async_stream::stream! {
        let mut parser = ResponsesSseParser::default();
        let mut byte_stream = response.bytes_stream();
        let mut pending_error = None;
        loop {
            let next = tokio::time::timeout(
                std::time::Duration::from_secs(300),
                codetether_codex_protocol::CodexProtocol::next_responses_sse_event(&mut byte_stream, &mut parser),
            ).await;
            let chunks = match next {
                Ok(Ok(Some(chunks))) => chunks,
                Ok(Ok(None)) => {
                    let error = pending_error.unwrap_or_else(|| {
                        "codex-retryable: stream closed before response.completed".into()
                    });
                    yield StreamChunk::Error(error);
                    return;
                }
                Ok(Err(error)) => {
                    yield StreamChunk::Error(format!("SSE stream error: {error}"));
                    return;
                }
                Err(_) => {
                    yield StreamChunk::Error("idle timeout waiting for SSE".into());
                    return;
                }
            };
            if chunks.is_empty() { yield StreamChunk::KeepAlive; }
            for chunk in chunks {
                match chunk {
                    StreamChunk::Error(error) => pending_error = Some(error),
                    done @ StreamChunk::Done { .. } => { yield done; return; }
                    other => yield other,
                }
            }
        }
    })
}
