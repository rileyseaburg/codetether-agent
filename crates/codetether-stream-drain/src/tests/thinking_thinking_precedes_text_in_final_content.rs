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
