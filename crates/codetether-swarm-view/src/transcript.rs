//! Convert the selected worker transcript into shared chat messages.
use super::transcript_role::message_type;
use codetether_chat_model::{ChatMessage, MessageType};

/// Convert the selected worker's history into ordinary chat messages.
///
/// # Arguments
/// * `state` - Current swarm monitor state.
/// * `selected_agent` - Selected worker name, if any.
///
/// # Returns
/// Worker header, history, fallback output and error messages, or none when absent.
///
/// # Examples
/// ```
/// let state = codetether_swarm_view::SwarmViewState::new();
/// assert!(codetether_swarm_view::active_messages(&state, None).is_none());
/// ```
pub fn active_messages(
    state: &crate::SwarmViewState,
    selected_agent: Option<&str>,
) -> Option<Vec<ChatMessage>> {
    let name = selected_agent?;
    let task = state
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
