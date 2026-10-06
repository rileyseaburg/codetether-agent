//! Tool-less native InvokeModel requests replaying tool history (TUI `/ask`)
//! must still declare `tools`, or Bedrock rejects the body with a 400.

use super::body_tests::req;
use crate::provider::bedrock::invoke::body::build_anthropic_messages_body;
use crate::provider::{ContentPart, Message, Role};

fn text(role: Role, text: &str) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}

#[test]
fn tool_history_without_tools_declares_native_tools() {
    let messages = vec![
        text(Role::User, "go"),
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: "call_hist".into(),
                name: "exec_command".into(),
                arguments: "{}".into(),
                thought_signature: None,
            }],
        },
        Message {
            role: Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: "call_hist".into(),
                content: "ok".into(),
            }],
        },
        text(Role::User, "side question"),
    ];
    let body = build_anthropic_messages_body(&req(messages, vec![]), "us.anthropic.claude-fable-5");
    assert_eq!(body["tools"][0]["name"], "exec_command", "{body}");
}

#[test]
fn plain_history_without_tools_omits_native_tools() {
    let body = build_anthropic_messages_body(
        &req(vec![text(Role::User, "hi")], vec![]),
        "us.anthropic.claude-fable-5",
    );
    assert!(body.get("tools").is_none(), "{body}");
}
