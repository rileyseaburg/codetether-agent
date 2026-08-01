use super::super::Parser;
use crate::provider::StreamChunk;

fn sse(json: &str) -> String {
    format!("data: {json}\n\n")
}

#[test]
fn later_argument_delta_reuses_first_chunks_id_by_index() {
    let mut parser = Parser::default();
    let first = sse(
        r#"{"choices":[{"delta":{"tool_calls":[{"index":0,"id":"call-1","function":{"name":"read","arguments":""}}]},"finish_reason":null}]}"#,
    );
    let args = sse(
        r#"{"choices":[{"delta":{"tool_calls":[{"index":0,"id":null,"function":{"name":null,"arguments":"{\"path\":\"x\"}"}}]},"finish_reason":null}]}"#,
    );
    let done = sse(r#"{"choices":[{"delta":{},"finish_reason":"tool_calls"}]}"#);
    let chunks = [
        parser.push(first.as_bytes()),
        parser.push(args.as_bytes()),
        parser.push(done.as_bytes()),
    ]
    .concat();

    assert!(matches!(
        &chunks[0], StreamChunk::ToolCallStart { id, name }
        if id == "call-1" && name == "read"
    ));
    assert!(matches!(
        &chunks[1], StreamChunk::ToolCallDelta { id, arguments_delta }
        if id == "call-1" && arguments_delta == "{\"path\":\"x\"}"
    ));
    assert!(!chunks.iter().any(|chunk| matches!(
        chunk, StreamChunk::ToolCallStart { name, .. } if name == "tool"
    )));
}
