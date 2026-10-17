#[test]
fn builds_responses_ws_create_event_for_tools() {
    let event = OpenAiCodexProvider::build_responses_ws_create_event(
        &ws_tool_request(),
        "gpt-5.4",
        Some(ThinkingLevel::High),
        None,
    );
    assert_eq!(
        event.get("type").and_then(Value::as_str),
        Some("response.create")
    );
    assert_eq!(event.get("model").and_then(Value::as_str), Some("gpt-5.4"));
    assert_eq!(event.get("store").and_then(Value::as_bool), Some(false));
    assert_eq!(
        event.get("max_output_tokens").and_then(Value::as_u64),
        Some(8192)
    );
    assert_eq!(
        event.get("tools").and_then(Value::as_array).map(Vec::len),
        Some(1)
    );
}
