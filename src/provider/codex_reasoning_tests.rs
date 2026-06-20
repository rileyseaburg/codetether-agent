//! Tests for [`super::push_if_reasoning`].

use super::push_if_reasoning;
use crate::provider::StreamChunk;
use serde_json::json;

#[test]
fn forwards_reasoning_text_delta() {
    let mut chunks = Vec::new();
    let ev = json!({"type": "response.reasoning_text.delta", "delta": "thinking..."});
    push_if_reasoning(Some("response.reasoning_text.delta"), &ev, &mut chunks);
    assert!(matches!(&chunks[..], [StreamChunk::Thinking(s)] if s == "thinking..."));
}

#[test]
fn forwards_summary_lifecycle_marker_as_empty_tick() {
    let mut chunks = Vec::new();
    let ev = json!({"type": "response.reasoning_summary_part.added"});
    push_if_reasoning(
        Some("response.reasoning_summary_part.added"),
        &ev,
        &mut chunks,
    );
    assert!(matches!(&chunks[..], [StreamChunk::Thinking(s)] if s.is_empty()));
}

#[test]
fn ignores_non_reasoning_events() {
    let mut chunks = Vec::new();
    let ev = json!({"type": "response.output_text.delta", "delta": "hi"});
    push_if_reasoning(Some("response.output_text.delta"), &ev, &mut chunks);
    assert!(chunks.is_empty());
}

#[test]
fn ignores_none_type() {
    let mut chunks = Vec::new();
    push_if_reasoning(None, &json!({}), &mut chunks);
    assert!(chunks.is_empty());
}
