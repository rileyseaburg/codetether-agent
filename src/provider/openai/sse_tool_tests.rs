//! Tests for raw-SSE reasoning frame parsing (tool calls + empty frames).

use super::sse_tests::parse;
use crate::provider::StreamChunk;

#[test]
fn tool_call_frame_emits_start_and_delta() {
    let mut produced = false;
    let chunks = parse(
        r#"{"choices":[{"delta":{"tool_calls":[{"index":0,"id":"call_1","function":{"name":"read","arguments":"{\"p\":1}"}}]}}]}"#,
        &mut produced,
    );
    assert!(produced);
    assert!(matches!(&chunks[0], StreamChunk::ToolCallStart { name, .. } if name == "read"));
    assert!(matches!(&chunks[1], StreamChunk::ToolCallDelta { .. }));
}

#[test]
fn empty_frame_produces_nothing() {
    let mut produced = false;
    let chunks = parse(r#"{"choices":[{"delta":{}}]}"#, &mut produced);
    assert!(!produced);
    assert!(chunks.is_empty());
}
