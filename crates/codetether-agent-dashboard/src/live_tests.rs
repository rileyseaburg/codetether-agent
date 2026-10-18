//! Live trace rendering tests.

use super::*;

#[test]
fn renders_activity_streaming_text_and_tool_events() {
    let trace = LiveTraceSnapshot {
        prompt: "inspect src/lib.rs".into(),
        entries: vec![LiveTraceEntry::ToolCall {
            name: "read".into(),
            arguments: "src/lib.rs".into(),
        }],
        streaming_text: Some("I found the module list".into()),
        activity: Some("Writing response…".into()),
    };
    let mut rows = Vec::new();

    append(&mut rows, &trace);

    let text = rows
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n");
    assert!(text.contains("LIVE"));
    assert!(text.contains("Writing response"));
    assert!(text.contains("TOOL · read"));
    assert!(text.contains("I found the module list"));
}
