//! Bind a fresh child session and its initial prompt to its task-owned checkout.

use super::{super::session_factory::system_message, workspace::Handoff};
use crate::provider::{ContentPart, Message, Role};
use crate::session::Session;

pub(super) fn bind(session: &mut Session, name: &str, instructions: &str, handoff: &Handoff) {
    session.metadata.directory = Some(handoff.workspace.clone());
    session.metadata.workspace_pinned = true;
    let text = system_message::build(name, instructions, session.metadata.directory.clone());
    // Called before fork inheritance: replace the factory's parent-cwd prompt.
    if let Some(message) = session.messages.first_mut() {
        *message = Message {
            role: Role::System,
            content: vec![ContentPart::Text { text }],
        };
    }
}

pub(super) fn remind(session: &mut Session, handoff: &Handoff) {
    // Place authoritative checkout guidance after inherited parent history.
    session.add_message(Message {
        role: Role::System,
        content: vec![ContentPart::Text {
            text: handoff.guidance(),
        }],
    });
}
