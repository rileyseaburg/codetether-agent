//! Workspace-bearing child system-message refresh.

use crate::provider::{ContentPart, Message, Role};
use crate::tool::agent::store::AgentEntry;

pub(super) fn refresh(entry: &mut AgentEntry) {
    let text = super::super::super::session_factory::system_message::build(
        &entry.name,
        &entry.instructions,
        entry.session.metadata.directory.clone(),
    );
    let message = Message {
        role: Role::System,
        content: vec![ContentPart::Text { text }],
    };
    if let Some(current) = entry
        .session
        .messages
        .iter_mut()
        .find(|message| message.role == Role::System)
    {
        *current = message;
        entry.session.summary_index = crate::session::index::SummaryIndex::new();
    } else {
        entry.session.add_message(message);
    }
}
