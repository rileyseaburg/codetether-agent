//! Test that the InvokeModel body uses the native Anthropic shape.

use super::body_tests::req;
use crate::provider::bedrock::invoke::body::build_anthropic_messages_body;
use crate::provider::{ContentPart, Message, Role, ToolDefinition};

#[test]
fn emits_native_tool_and_message_shape() {
    let r = req(
        vec![
            Message {
                role: Role::System,
                content: vec![ContentPart::Text { text: "sys".into() }],
            },
            Message {
                role: Role::User,
                content: vec![ContentPart::Text { text: "hi".into() }],
            },
        ],
        vec![ToolDefinition {
            name: "bash".into(),
            description: "run".into(),
            parameters: serde_json::json!({"type": "object"}),
        }],
    );
    let body = build_anthropic_messages_body(&r, "us.anthropic.claude-fable-5");
    assert_eq!(body["tools"][0]["name"], "bash");
    assert_eq!(body["tools"][0]["input_schema"]["type"], "object");
    assert!(body["tools"][0].get("toolSpec").is_none());
    assert_eq!(body["messages"][0]["content"][0]["type"], "text");
    assert_eq!(body["messages"][0]["content"][0]["text"], "hi");
    assert_eq!(body["system"][0]["type"], "text");
}
