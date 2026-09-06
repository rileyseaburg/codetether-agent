//! Transport-only image follow-ups; stored history and tool roles stay intact.

use crate::provider::{ContentPart, Message, Role};

pub(in crate::provider::openai) fn transport_history(messages: &[Message]) -> Vec<Message> {
    let mut result = Vec::new();
    let mut images = Vec::new();
    for message in messages {
        if message.role == Role::Tool {
            images.extend(followups(message));
        } else {
            result.append(&mut images);
        }
        result.push(message.clone());
    }
    result.append(&mut images);
    result
}

fn followups(message: &Message) -> Vec<Message> {
    let mut call_id = message.content.iter().find_map(|part| match part {
        ContentPart::ToolResult { tool_call_id, .. } => Some(tool_call_id),
        _ => None,
    });
    let mut result = Vec::new();
    for part in &message.content {
        match part {
            ContentPart::ToolResult { tool_call_id, .. } => call_id = Some(tool_call_id),
            ContentPart::Image { .. } => {
                let label = call_id.map_or_else(
                    || "Image attached to tool output (missing tool_call_id).".to_owned(),
                    |id| format!("Image attached to tool output for tool_call_id: {id}"),
                );
                result.push(Message {
                    role: Role::User,
                    content: vec![ContentPart::Text { text: label }, part.clone()],
                });
            }
            _ => {}
        }
    }
    result
}
