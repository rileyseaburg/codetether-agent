use codetether_provider_core::{ContentPart, Message, Role};

pub(super) fn text(role: Role, s: &str) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text {
            text: s.to_string(),
        }],
    }
}

pub(super) fn tool_call(id: &str, name: &str) -> Message {
    Message {
        role: Role::Assistant,
        content: vec![ContentPart::ToolCall {
            id: id.to_string(),
            name: name.to_string(),
            arguments: "{}".to_string(),
            thought_signature: None,
        }],
    }
}

pub(super) fn tool_result(id: &str, body: &str) -> Message {
    Message {
        role: Role::Tool,
        content: vec![ContentPart::ToolResult {
            tool_call_id: id.to_string(),
            content: body.to_string(),
        }],
    }
}
