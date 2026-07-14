#[test]
fn responses_sse_parser_flushes_final_event_without_trailing_blank_line() {
    let mut parser = ResponsesSseParser::default();
    let bytes = br#"data: {"type":"response.output_item.done","item":{"type":"function_call","id":"fc_3","call_id":"call_3","name":"read","arguments":"{\"path\":\"src/lib.rs\"}"}}"#;

    let first = OpenAiCodexProvider::parse_responses_sse_bytes(&mut parser, bytes);
    assert!(first.is_empty());

    let flushed = OpenAiCodexProvider::finish_responses_sse_parser(&mut parser);
    assert_eq!(flushed.len(), 3);
    match &flushed[0] {
        StreamChunk::ToolCallStart { id, name } => {
            assert_eq!(id, "call_3");
            assert_eq!(name, "read");
        }
        other => panic!("expected tool start, got {other:?}"),
    }
    match &flushed[1] {
        StreamChunk::ToolCallDelta {
            id,
            arguments_delta,
        } => {
            assert_eq!(id, "call_3");
            assert_eq!(arguments_delta, "{\"path\":\"src/lib.rs\"}");
        }
        other => panic!("expected tool delta, got {other:?}"),
    }
    match &flushed[2] {
        StreamChunk::ToolCallEnd { id } => assert_eq!(id, "call_3"),
        other => panic!("expected tool end, got {other:?}"),
    }
}
