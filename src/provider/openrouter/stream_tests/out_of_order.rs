use super::super::Parser;
use crate::provider::StreamChunk;

fn sse(json: &str) -> String {
    format!("data: {json}\n\n")
}

#[test]
fn arguments_arriving_before_identity_are_buffered() {
    let mut parser = Parser::default();
    let args = sse(
        r#"{"choices":[{"delta":{"tool_calls":[{"index":2,"function":{"arguments":"{}"}}]}}]}"#,
    );
    let identity = sse(
        r#"{"choices":[{"delta":{"tool_calls":[{"index":2,"id":"late","function":{"name":"grep"}}]}}]}"#,
    );
    assert!(parser.push(args.as_bytes()).is_empty());
    let chunks = parser.push(identity.as_bytes());
    assert!(matches!(
        &chunks[..],
        [StreamChunk::ToolCallStart { id, name }, StreamChunk::ToolCallDelta { id: delta_id, arguments_delta }]
        if id == "late" && name == "grep" && delta_id == "late" && arguments_delta == "{}"
    ));
}
