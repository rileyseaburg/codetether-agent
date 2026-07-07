//! Tests for [`SseChunkStream`] premature-EOF handling.

use futures::StreamExt;

use super::BlockParser;
use super::SseChunkStream;
use crate::provider::StreamChunk;

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
async fn premature_eof_without_done_yields_transient_error() {
    let sse = [
        "event: content_block_delta",
        r#"data: {"type":"content_block_delta","index":0,"delta":{"type":"text_delta","text":"hi"}}"#,
        "",
    ];
    let chunks: Vec<_> = Box::pin(stream_from_sse(&sse)).collect().await;
    let has_error = chunks
        .iter()
        .any(|c| matches!(c, StreamChunk::Error(m) if m.contains("premature EOF")));
    assert!(has_error, "expected premature-EOF error, got {chunks:?}");
    let dones: Vec<_> = chunks
        .iter()
        .filter(|c| matches!(c, StreamChunk::Done { .. }))
        .collect();
    assert!(dones.is_empty(), "spurious Done emitted: {dones:?}");
}
