//! Original live-event truncation regression cases.

use super::event::*;

#[test]
fn keeps_small_output() {
    assert_eq!(bounded_tool_output("ok"), "ok");
}

#[test]
fn truncates_large_output() {
    let text = "a".repeat(TOOL_OUTPUT_EVENT_MAX_BYTES + 1);
    let bounded = bounded_tool_output(&text);
    assert!(bounded.len() < text.len());
    assert!(bounded.contains("truncated for live TUI event"));
}

#[test]
fn truncates_large_arguments() {
    let text = "b".repeat(TOOL_ARGUMENTS_EVENT_MAX_BYTES + 1);
    let bounded = bounded_tool_arguments(&text);
    assert!(bounded.len() < text.len());
    assert!(bounded.contains("tool arguments"));
}
