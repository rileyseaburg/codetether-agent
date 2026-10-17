//! Regression coverage for convert_messages_serializes_tool_arguments_as_json_string.
use crate::*;

#[test]
fn convert_messages_serializes_tool_arguments_as_json_string() {
    let messages = vec![Message {
        role: Role::Assistant,
        content: vec![ContentPart::ToolCall {
            id: "call_1".to_string(),
            name: "bash".to_string(),
            arguments: "{\"command\":\"ls\"}".to_string(),
            thought_signature: None,
        }],
    }];

    let converted = Glm5Provider::convert_messages(&messages);
    let args = converted[0]["tool_calls"][0]["function"]["arguments"]
        .as_str()
        .expect("arguments must be a string");

    assert_eq!(args, r#"{"command":"ls"}"#);
}
