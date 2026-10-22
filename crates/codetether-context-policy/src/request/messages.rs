//! Stable system and derived-history prefix assembly.

use codetether_provider_core::{ContentPart, Message, Role};

pub fn build(system_prompt: &str, mut derived: Vec<Message>) -> Vec<Message> {
    let (stable, state) = codetether_provider_core::runtime_context::split(system_prompt);
    if let Some(state) = state {
        let insertion = evidence_boundary(&derived);
        derived.insert(
            insertion,
            Message {
                role: Role::User,
                content: vec![ContentPart::Text { text: state }],
            },
        );
    }
    let mut messages = vec![Message {
        role: Role::System,
        content: vec![ContentPart::Text { text: stable }],
    }];
    messages.extend(derived);
    messages
}

// Preserve all completed history. A fresh user request stays last; supplemental
// evidence follows completed tool rounds when the user request is earlier.
pub(super) fn evidence_boundary(messages: &[Message]) -> usize {
    if messages
        .last()
        .is_some_and(|message| message.role == Role::User)
    {
        messages.len() - 1
    } else {
        messages.len()
    }
}
