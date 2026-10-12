//! Conversation history conversion to the Z.AI chat schema.
use crate::{ContentPart, Message, Role};
use serde_json::{Value, json};

pub(crate) fn convert(messages: &[Message], include_reasoning: bool) -> Vec<Value> {
    messages
        .iter()
        .flat_map(|message| match message.role {
            Role::Assistant => vec![crate::assistant_message::convert(
                message,
                include_reasoning,
            )],
            Role::Tool => message
                .content
                .iter()
                .filter_map(|part| match part {
                    ContentPart::ToolResult {
                        tool_call_id,
                        content,
                    } => Some(json!({
                        "role": "tool", "tool_call_id": tool_call_id, "content": content
                    })),
                    _ => None,
                })
                .collect(),
            Role::System | Role::Developer | Role::User => {
                let text = message
                    .content
                    .iter()
                    .filter_map(|part| match part {
                        ContentPart::Text { text } => Some(text.as_str()),
                        _ => None,
                    })
                    .collect::<String>();
                let role = if matches!(message.role, Role::System | Role::Developer) {
                    "system"
                } else {
                    "user"
                };
                vec![json!({"role": role, "content": text})]
            }
        })
        .collect()
}
