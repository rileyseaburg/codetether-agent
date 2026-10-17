//! Workspace-bearing child system-message refresh.

use crate::ChildSession;
use codetether_agent_registry::AgentEntry;
use codetether_provider_core::{ContentPart, Message, Role};

pub(super) fn refresh<S: ChildSession>(entry: &mut AgentEntry<S>) {
    let text = codetether_agent_messages::system_message(
        &entry.name,
        &entry.instructions,
        entry.session.settings().directory.clone(),
    );
    let message = Message {
        role: Role::System,
        content: vec![ContentPart::Text { text }],
    };
    if let Some(current) = entry
        .session
        .messages_mut()
        .find(|message| message.role == Role::System)
    {
        *current = message;
        entry.session.reset_summary_index();
    } else {
        entry.session.add_message(message);
    }
}
