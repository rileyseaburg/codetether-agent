//! Bind a fresh child session and its initial prompt to its task-owned checkout.

use crate::ChildSession;
use codetether_child_workspace::Handoff;
use codetether_provider_core::{ContentPart, Message, Role};

pub(super) fn bind<S: ChildSession>(
    session: &mut S,
    name: &str,
    instructions: &str,
    handoff: &Handoff,
) {
    *session.settings().directory = Some(handoff.workspace.clone());
    session.set_workspace_pinned(true);
    let text = codetether_agent_messages::system_message(
        name,
        instructions,
        session.settings().directory.clone(),
    );
    // Called before fork inheritance: replace the factory's parent-cwd prompt.
    if let Some(message) = session.first_message_mut() {
        *message = Message {
            role: Role::System,
            content: vec![ContentPart::Text { text }],
        };
    }
}

pub(super) fn remind<S: ChildSession>(session: &mut S, handoff: &Handoff) {
    // Place authoritative checkout guidance after inherited parent history.
    session.add_message(Message {
        role: Role::System,
        content: vec![ContentPart::Text {
            text: handoff.guidance(),
        }],
    });
}
