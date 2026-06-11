//! Tests for [`super::delta::handle_block_delta`], including the
//! reasoning-delta path that previously dropped extended-thinking output.

use super::StreamState;
use super::delta::handle_block_delta;
use crate::provider::StreamChunk;
use serde_json::json;

#[test]
fn reasoning_delta_yields_thinking_chunk() {
    let state = StreamState::default();
    let body = json!({
        "contentBlockIndex": 0,
        "delta": {"reasoningContent": {"text": "pondering"}}
    });
    let out = handle_block_delta(&state, &body);
    assert_eq!(out.len(), 1);
    assert!(matches!(&out[0], StreamChunk::Thinking(t) if t == "pondering"));
}

#[test]
fn text_delta_yields_text_chunk() {
    let state = StreamState::default();
    let body = json!({"contentBlockIndex": 0, "delta": {"text": "hi"}});
    let out = handle_block_delta(&state, &body);
    assert!(matches!(&out[0], StreamChunk::Text(t) if t == "hi"));
}

#[test]
fn tool_use_delta_yields_arguments_for_open_block() {
    let mut state = StreamState::default();
    state.open_tool_blocks.insert(2, "tool-1".to_string());
    let body = json!({
        "contentBlockIndex": 2,
        "delta": {"toolUse": {"input": "{\"a\":"}}
    });
    let out = handle_block_delta(&state, &body);
    assert!(matches!(
        &out[0],
        StreamChunk::ToolCallDelta { id, arguments_delta }
            if id == "tool-1" && arguments_delta == "{\"a\":"
    ));
}

#[test]
fn signature_only_reasoning_delta_yields_heartbeat() {
    let state = StreamState::default();
    let body = json!({"delta": {"reasoningContent": {"signature": "sig"}}});
    let out = handle_block_delta(&state, &body);
    assert!(matches!(&out[0], StreamChunk::Thinking(t) if t.is_empty()));
}
