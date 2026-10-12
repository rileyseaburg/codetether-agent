use codetether_provider_core::{ContentPart, ToolCallRef};

#[test]
fn tool_call_view_borrows_raw_arguments_and_signature() {
    let part = ContentPart::ToolCall {
        id: "call-1".into(),
        name: "lookup".into(),
        arguments: "{ \"items\": [1, 2] }\n".into(),
        thought_signature: Some("signed-thought".into()),
    };
    let view: ToolCallRef<'_> = part.as_tool_call().unwrap();
    assert_eq!(view.id, "call-1");
    assert_eq!(view.name, "lookup");
    assert_eq!(view.arguments, "{ \"items\": [1, 2] }\n");
    assert_eq!(view.thought_signature, Some("signed-thought"));
    if let ContentPart::ToolCall {
        arguments,
        thought_signature,
        ..
    } = &part
    {
        assert!(std::ptr::eq(view.arguments, arguments.as_str()));
        assert!(std::ptr::eq(
            view.thought_signature.unwrap(),
            thought_signature.as_deref().unwrap()
        ));
    }
}

#[test]
fn missing_signature_is_preserved_and_non_calls_have_no_view() {
    let part = ContentPart::ToolCall {
        id: "call-1".into(),
        name: "lookup".into(),
        arguments: "{}".into(),
        thought_signature: None,
    };
    assert_eq!(part.as_tool_call().unwrap().thought_signature, None);
    let text = ContentPart::Text {
        text: "hello".into(),
    };
    assert!(text.as_tool_call().is_none());
    assert!(
        ContentPart::ToolResult {
            tool_call_id: "call-1".into(),
            content: "ok".into(),
        }
        .as_tool_call()
        .is_none()
    );
}
