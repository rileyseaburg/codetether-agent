//! Tool-less requests (the TUI `/ask` side question) that replay tool history
//! must still carry `toolConfig`, or Bedrock answers with a permanent 400:
//! "The toolConfig field must be defined when using toolUse and toolResult".

use super::support::{call, request};
use crate::provider::bedrock::build_converse_body;
use crate::provider::{ContentPart, Message, Role};

fn text(role: Role, text: &str) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}

#[test]
fn tool_history_without_tools_still_declares_tool_config() {
    let body = build_converse_body(
        &request(vec![
            text(Role::User, "go"),
            Message {
                role: Role::Assistant,
                content: vec![call("call_hist")],
            },
            Message {
                role: Role::Tool,
                content: vec![ContentPart::ToolResult {
                    tool_call_id: "call_hist".into(),
                    content: "ok".into(),
                }],
            },
            text(Role::User, "side question"),
        ]),
        "us.anthropic.claude-opus-4-7",
    );
    let tools = body["toolConfig"]["tools"].as_array().expect("toolConfig");
    assert_eq!(tools[0]["toolSpec"]["name"], "exec_command", "{body}");
}

#[test]
fn plain_history_without_tools_omits_tool_config() {
    let body = build_converse_body(
        &request(vec![text(Role::User, "hi")]),
        "us.anthropic.claude-opus-4-7",
    );
    assert!(body.get("toolConfig").is_none(), "{body}");
}
