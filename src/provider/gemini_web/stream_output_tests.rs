use super::chunks;
use crate::provider::{StreamChunk, ToolDefinition};

fn tool(name: &str) -> ToolDefinition {
    ToolDefinition {
        name: name.into(),
        description: String::new(),
        parameters: serde_json::json!({"type": "object"}),
    }
}

#[path = "stream_output_tests/edges.rs"]
mod edges;
#[path = "stream_output_tests/forgery.rs"]
mod forgery;
#[path = "stream_output_tests/partial.rs"]
mod partial;

#[test]
fn converts_arguments_first_markup_into_stream_tool_events() {
    let text = r#"<tool_call>{"arguments":{"cmd":"pwd"},"name":"exec_command"}</tool_call>"#;
    let chunks = chunks(text, &[tool("exec_command")], &[]).unwrap();
    assert!(matches!(
        &chunks[0],
        StreamChunk::ToolCallStart { name, .. } if name == "exec_command"
    ));
    assert!(matches!(
        &chunks[1],
        StreamChunk::ToolCallDelta { arguments_delta, .. }
            if arguments_delta == r#"{"cmd":"pwd"}"#
    ));
    assert!(matches!(&chunks[2], StreamChunk::ToolCallEnd { .. }));
    assert!(matches!(&chunks[3], StreamChunk::Done { .. }));
}

#[test]
fn removes_tool_markup_from_visible_text() {
    let text = r#"Checking now.<tool_call>{"name":"read","arguments":{}}</tool_call>"#;
    let chunks = chunks(text, &[tool("read")], &[]).unwrap();
    assert!(matches!(&chunks[0], StreamChunk::Text(text) if text == "Checking now."));
    assert!(
        !chunks.iter().any(|chunk| {
            matches!(chunk, StreamChunk::Text(text) if text.contains("<tool_call>"))
        })
    );
}
