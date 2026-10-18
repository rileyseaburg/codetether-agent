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
