//! Tests for raw-SSE reasoning frame parsing (content + reasoning).

use super::sse_frame::frame_chunks;
use super::sse_types::SseResponse;
use crate::provider::StreamChunk;

pub(super) fn parse(data: &str, produced: &mut bool) -> Vec<StreamChunk> {
    let resp: SseResponse = serde_json::from_str(data).expect("frame parses");
    frame_chunks(&resp, produced)
}

#[test]
fn reasoning_only_frame_yields_thinking_and_marks_produced() {
    let mut produced = false;
    let chunks = parse(
        r#"{"choices":[{"delta":{"reasoning_content":"let me think"}}]}"#,
        &mut produced,
    );
    assert!(produced, "reasoning content must count as produced output");
    assert!(matches!(&chunks[0], StreamChunk::Thinking(t) if t == "let me think"));
}

#[test]
fn reasoning_alias_field_is_accepted() {
    let mut produced = false;
    let chunks = parse(
        r#"{"choices":[{"delta":{"reasoning":"alt field"}}]}"#,
        &mut produced,
    );
    assert!(matches!(&chunks[0], StreamChunk::Thinking(t) if t == "alt field"));
}

#[test]
fn content_frame_yields_text() {
    let mut produced = false;
    let chunks = parse(
        r#"{"choices":[{"delta":{"content":"hello"}}]}"#,
        &mut produced,
    );
    assert!(produced);
    assert!(matches!(&chunks[0], StreamChunk::Text(t) if t == "hello"));
}
