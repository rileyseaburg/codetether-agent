//! Map worker event roles to chat-message classifications.
use codetether_chat_model::MessageType;

pub(super) fn message_type(entry: &crate::AgentMessageEntry) -> MessageType {
    if entry.is_tool_call {
        return MessageType::ToolCall {
            name: "swarm".into(),
            arguments: entry.content.clone(),
        };
    }
    match entry.role.as_str() {
        "user" => MessageType::User,
        "assistant" => MessageType::Assistant,
        "error" => MessageType::Error,
        _ => MessageType::System,
    }
}
