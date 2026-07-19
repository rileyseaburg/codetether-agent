#[test]
fn responses_sse_parser_buffers_split_tool_call_events() {
    let mut parser = ResponsesSseParser::default();
    let first_bytes = br#"data: {"type":"response.output_item.added","item":{"type":"function_call","id":"fc_1","call_id":"call_1","name":"bash"}}

da"#;
    let first = OpenAiCodexProvider::parse_responses_sse_bytes(&mut parser, first_bytes);
    assert_tool_start(&first);
    let remaining_bytes = br#"ta: {"type":"response.function_call_arguments.delta","item_id":"fc_1","delta":"{\"command\":"}

data: {"type":"response.function_call_arguments.delta","item_id":"fc_1","delta":"\"ls\"}"}

data: {"type":"response.output_item.done","item":{"type":"function_call","id":"fc_1","call_id":"call_1","name":"bash","arguments":"{\"command\":\"ls\"}"}}

data: {"type":"response.completed","response":{"id":"resp-1","usage":{"input_tokens":11,"output_tokens":7,"total_tokens":18}}}

"#;
    let remaining = OpenAiCodexProvider::parse_responses_sse_bytes(&mut parser, remaining_bytes);
    assert_tool_completion(&remaining);
}
