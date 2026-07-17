//! Tests for thinking-chunk accumulation in the stream collector.
//!
//! Guards against the Bedrock regression where a thinking-only completion
//! (e.g. Claude Fable 5 extended thinking) collapsed into an empty
//! assistant message.

use super::collect_stream_completion_with_events;
use crate::provider::{ContentPart, StreamChunk};
use crate::session::SessionEvent;
use futures::stream;

#[tokio::test]
async fn thinking_only_stream_produces_thinking_content() {
    let s = Box::pin(stream::iter(vec![
        StreamChunk::Thinking("deep".to_string()),
        StreamChunk::Thinking(" thought".to_string()),
        StreamChunk::Done { usage: None },
    ]));
    let resp = collect_stream_completion_with_events(s, None)
        .await
        .unwrap();
    assert!(matches!(
        &resp.message.content[0],
        ContentPart::Thinking { text, .. } if text == "deep thought"
    ));
}

#[tokio::test]
async fn thinking_precedes_text_in_final_content() {
    let s = Box::pin(stream::iter(vec![
        StreamChunk::Thinking("plan".to_string()),
        StreamChunk::Text("answer".to_string()),
        StreamChunk::Done { usage: None },
    ]));
    let resp = collect_stream_completion_with_events(s, None)
        .await
        .unwrap();
    assert_eq!(resp.message.content.len(), 2);
    assert!(matches!(
        &resp.message.content[0],
        ContentPart::Thinking { text, .. } if text == "plan"
    ));
    assert!(matches!(
        &resp.message.content[1],
        ContentPart::Text { text } if text == "answer"
    ));
}

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
