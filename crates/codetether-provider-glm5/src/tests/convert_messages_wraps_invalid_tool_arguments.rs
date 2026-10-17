//! Regression coverage for convert_messages_wraps_invalid_tool_arguments.
use crate::*;

#[test]
fn convert_messages_wraps_invalid_tool_arguments() {
    let messages = vec![Message {
        role: Role::Assistant,
        content: vec![ContentPart::ToolCall {
            id: "call_1".to_string(),
            name: "bash".to_string(),
            arguments: "command=ls".to_string(),
            thought_signature: None,
        }],
    }];

    let converted = Glm5Provider::convert_messages(&messages);
    let args = converted[0]["tool_calls"][0]["function"]["arguments"]
        .as_str()
        .expect("arguments must be a string");
    let parsed: Value = serde_json::from_str(args).expect("wrapped arguments must be valid JSON");

    assert_eq!(parsed, json!({"input": "command=ls"}));
}
