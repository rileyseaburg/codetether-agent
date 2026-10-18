//! Original single-drain regression case.
//! Tests for thinking-chunk accumulation in the stream collector.
//!
//! Guards against the Bedrock regression where a thinking-only completion
//! (e.g. Claude Fable 5 extended thinking) collapsed into an empty
//! assistant message.

use super::collect_stream_completion_with_events;

use codetether_provider_core::{ContentPart, StreamChunk};

use super::SessionEvent;

use futures::stream;

#[tokio::test]
async fn empty_thinking_emits_activity_without_content() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(1);
    let s = Box::pin(stream::iter(vec![StreamChunk::Thinking(String::new())]));
    let err = collect_stream_completion_with_events(s, Some(&tx))
        .await
        .unwrap_err();
    // Stream ended before a `Done` chunk with no committed content: the drain
    // classifies this as a premature termination, not a clean empty finish.
    assert!(err.to_string().contains("availability"));
    assert!(matches!(rx.try_recv(), Ok(SessionEvent::Thinking)));
}
