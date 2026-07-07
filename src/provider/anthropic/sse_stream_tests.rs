//! Tests for [`SseChunkStream`] EOF / premature-close handling.

use futures::StreamExt;

use super::BlockParser;
use super::SseChunkStream;
use crate::provider::StreamChunk;

/// Build a minimal fake `SseChunkStream` by injecting a pre-built inner
/// byte stream so we can exercise the poll impl without a real HTTP response.
fn stream_from_sse(lines: &[&str]) -> SseChunkStream {
    use bytes::Bytes;
    let payload: String = lines.join("\n");
    let bytes_stream = futures::stream::iter(
        payload
            .into_bytes()
            .chunks(64)
            .map(|c| Ok::<Bytes, reqwest::Error>(Bytes::copy_from_slice(c)))
            .collect::<Vec<_>>(),
    );
    SseChunkStream {
        inner: Box::pin(bytes_stream),
        buffer: String::new(),
        pending_event: None,
        saw_done: false,
        blocks: BlockParser::new(),
    }
}

#[tokio::test]
async fn clean_eof_after_message_delta_ends_stream_normally() {
    // message_delta carries Done; byte-stream close should NOT add a second Done.
    let sse = [
        "event: message_delta",
        r#"data: {"type":"message_delta","usage":{"output_tokens":5}}"#,
        "",
    ];
    let chunks: Vec<_> = Box::pin(stream_from_sse(&sse)).collect().await;
    let dones: usize = chunks
        .iter()
        .filter(|c| matches!(c, StreamChunk::Done { .. }))
        .count();
    let errors: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c, StreamChunk::Error(_)))
        .collect();
    assert_eq!(dones, 1, "expected exactly one Done, got {chunks:?}");
    assert!(errors.is_empty(), "unexpected errors: {errors:?}");
}
