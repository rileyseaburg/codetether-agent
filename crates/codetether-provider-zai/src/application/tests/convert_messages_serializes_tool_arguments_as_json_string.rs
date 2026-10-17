//! Migrated application provider regression.
use super::*;
#[test]
fn convert_messages_serializes_tool_arguments_as_json_string() {
    let messages = vec![Message {
        role: Role::Assistant,
        content: vec![ContentPart::ToolCall {
            id: "call_1".to_string(),
            name: "get_weather".to_string(),
            arguments: "{\"city\":\"Beijing\".. }".to_string(),
            thought_signature: None,
        }],
    }];

    let converted = ZaiProvider::convert_messages(&messages, true);
    let args = converted[0]["tool_calls"][0]["function"]["arguments"]
        .as_str()
        .expect("arguments must be a string");
    let parsed: Value =
        serde_json::from_str(args).expect("arguments string must contain valid JSON");

    assert_eq!(parsed, json!({"city":"Beijing"}));
}
