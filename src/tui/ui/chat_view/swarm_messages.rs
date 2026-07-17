//! Chat-message projection for the swarm worker selected by the agent rail.

use crate::tui::app::state::AppState;
use crate::tui::chat::message::{ChatMessage, MessageType};

pub(super) fn active(state: &AppState) -> Option<Vec<ChatMessage>> {
    let name = state.active_spawned_agent.as_deref()?;
    let task = state
        .swarm
        .subtasks
        .iter()
        .find(|task| task.agent_name.as_deref() == Some(name))?;
    let header = format!(
        "Swarm agent: @{name}\nTask: {}\nSession: {}",
        task.name, task.id
    );
    let mut messages = vec![ChatMessage::new(MessageType::System, header)];
    messages.extend(
        task.messages
            .iter()
            .map(|entry| ChatMessage::new(message_type(entry), entry.content.clone())),
    );
    if task.messages.is_empty()
        && let Some(output) = task.output.as_deref()
    {
        messages.push(ChatMessage::new(MessageType::Assistant, output));
    }
    if let Some(error) = task.error.as_deref() {
        messages.push(ChatMessage::new(MessageType::Error, error));
    }
    Some(messages)
}

fn message_type(entry: &crate::tui::swarm_view::AgentMessageEntry) -> MessageType {
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
