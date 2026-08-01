//! Tests for Z.AI stream termination at `data: [DONE]`.

#[path = "zai_stream_done_fixtures.rs"]
mod fixtures;
#[path = "zai_stream_done_tool_tests.rs"]
mod tool;

use super::finish;
use crate::provider::StreamChunk;
use crate::provider::zai::stream_output::OutputSeen;
use std::collections::HashMap;

#[test]
fn buffered_text_is_flushed_before_done() {
    let mut chunks = Vec::new();
    let mut text = String::from("final answer");
    let mut seen = OutputSeen::default();
    let mut states = HashMap::new();

    finish(
        &mut chunks,
        &mut text,
        &mut seen,
        &mut states,
        5,
        None,
        None,
    );

    assert!(seen.text, "flushed text counts as output");
    assert!(matches!(&chunks[0], StreamChunk::Text(t) if t == "final answer"));
    assert!(matches!(chunks.last(), Some(StreamChunk::Done { .. })));
    assert!(!chunks.iter().any(|c| matches!(c, StreamChunk::Error(_))));
}

#[test]
fn truly_empty_stream_still_reports_error() {
    let mut chunks = Vec::new();
    let mut text = String::new();
    let mut seen = OutputSeen::default();
    let mut states = HashMap::new();

    finish(
        &mut chunks,
        &mut text,
        &mut seen,
        &mut states,
        0,
        None,
        None,
    );

    assert!(chunks.iter().any(|c| matches!(c, StreamChunk::Error(_))));
}
