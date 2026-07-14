#[test]
fn responses_sse_parser_falls_back_to_done_item_arguments() {
    let mut parser = ResponsesSseParser::default();
    let bytes = br#"data: {"type":"response.output_item.done","item":{"type":"function_call","id":"fc_2","call_id":"call_2","name":"read","arguments":"{\"path\":\"src/main.rs\"}"}}

"#;

    let chunks = OpenAiCodexProvider::parse_responses_sse_bytes(&mut parser, bytes);
    assert_eq!(chunks.len(), 3);
    match &chunks[0] {
        StreamChunk::ToolCallStart { id, name } => {
            assert_eq!(id, "call_2");
            assert_eq!(name, "read");
        }
        other => panic!("expected tool start, got {other:?}"),
    }
    match &chunks[1] {
        StreamChunk::ToolCallDelta {
            id,
            arguments_delta,
        } => {
            assert_eq!(id, "call_2");
            assert_eq!(arguments_delta, "{\"path\":\"src/main.rs\"}");
        }
        other => panic!("expected tool delta, got {other:?}"),
    }
    match &chunks[2] {
        StreamChunk::ToolCallEnd { id } => assert_eq!(id, "call_2"),
        other => panic!("expected tool end, got {other:?}"),
    }
}
