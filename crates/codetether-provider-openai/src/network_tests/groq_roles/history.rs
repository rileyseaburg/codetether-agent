//! Synthetic tool round-trip with corrective developer guidance.
use super::*;

fn text(role: Role, value: &str) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text { text: value.into() }],
    }
}
pub(super) fn input(model: &str) -> CompletionRequest {
    let mut input = request(model);
    input.messages = vec![
        text(Role::System, "Original instructions"),
        text(Role::Developer, "Initial guidance"),
        text(Role::User, "Run probe"),
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: "call-probe".into(),
                name: "probe".into(),
                arguments: "{}".into(),
                thought_signature: None,
            }],
        },
        Message {
            role: Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: "call-probe".into(),
                content: "OK".into(),
            }],
        },
        text(Role::Developer, "Continue briefly"),
    ];
    input.tools.push(ToolDefinition {
        name: "probe".into(),
        description: "fixture".into(),
        parameters: json!({"type":"object"}),
    });
    input
}
