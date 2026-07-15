//! Tests for compact in-flight trace updates.

use super::live_trace::*;
use crate::session::SessionEvent;

#[test]
fn cumulative_text_chunks_replace_the_live_draft() {
    begin("trace-text", "inspect".into());
    observe("trace-text", &SessionEvent::TextChunk("hel".into()));
    observe("trace-text", &SessionEvent::TextChunk("hello".into()));

    let trace = snapshot("trace-text").unwrap();
    assert_eq!(trace.streaming_text.as_deref(), Some("hello"));
    assert_eq!(trace.activity.as_deref(), Some("Writing response…"));
    clear("trace-text");
}

#[test]
fn tool_activity_remains_visible_until_completion() {
    begin("trace-tool", "inspect".into());
    observe(
        "trace-tool",
        &SessionEvent::ToolCallStart {
            tool_call_id: "1".into(),
            name: "read".into(),
            arguments: "src/lib.rs".into(),
        },
    );
    observe(
        "trace-tool",
        &SessionEvent::ToolHeartbeat {
            tool_call_id: "1".into(),
            name: "read".into(),
            elapsed_secs: 10,
        },
    );

    let trace = snapshot("trace-tool").unwrap();
    assert_eq!(trace.activity.as_deref(), Some("Running read · 10s"));
    assert!(matches!(trace.entries[0], LiveTraceEntry::ToolCall { .. }));
    clear("trace-tool");
}
