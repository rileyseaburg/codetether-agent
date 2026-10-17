//! Migrated application provider regression.
use super::*;
#[test]
fn convert_messages_uses_null_content_for_empty_assistant_tool_calls() {
    let messages = vec![Message {
        role: Role::Assistant,
        content: vec![ContentPart::ToolCall {
            id: "call_1".to_string(),
            name: "get_weather".to_string(),
            arguments: r#"{"city":"Beijing"}"#.to_string(),
            thought_signature: None,
        }],
    }];

    let converted = ZaiProvider::convert_messages(&messages, false);

    assert!(converted[0].get("tool_calls").is_some());
    assert_eq!(converted[0]["content"], Value::Null);
}
