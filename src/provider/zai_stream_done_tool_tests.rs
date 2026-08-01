//! Tests for tool-call closing when a Z.AI stream terminates.

use super::super::finish;
use super::fixtures::{open_tool_state, tool_only};
use crate::provider::StreamChunk;

/// Runs `finish` over an open tool call with no buffered text.
fn close_open_call(finished: bool, reason: Option<&str>) -> Vec<StreamChunk> {
    let mut chunks = Vec::new();
    let (mut text, mut seen) = (String::new(), tool_only());
    let mut states = open_tool_state();
    states.get_mut(&0).expect("state present").finished = finished;
    finish(
        &mut chunks,
        &mut text,
        &mut seen,
        &mut states,
        99,
        reason,
        None,
    );
    chunks
}

#[test]
fn tool_only_turn_closes_call_and_reports_no_error() {
    // The live failure: 99 frames, a single bash tool call, no finish_reason.
    let chunks = close_open_call(false, None);

    assert!(
        !chunks.iter().any(|c| matches!(c, StreamChunk::Error(_))),
        "a tool-only turn is complete, not an empty stream"
    );
    assert!(
        chunks
            .iter()
            .any(|c| matches!(c, StreamChunk::ToolCallEnd { id } if id == "call_1")),
        "an open tool call must be closed at [DONE]"
    );
}

#[test]
fn already_finished_tool_call_is_not_closed_twice() {
    let chunks = close_open_call(true, Some("tool_calls"));

    assert!(
        !chunks
            .iter()
            .any(|c| matches!(c, StreamChunk::ToolCallEnd { .. })),
        "closing is idempotent"
    );
}
