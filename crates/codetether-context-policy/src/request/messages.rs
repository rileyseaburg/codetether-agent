//! Stable system and derived-history prefix assembly.

use codetether_provider_core::{ContentPart, Message, Role};

pub fn build(system_prompt: &str, derived: Vec<Message>) -> Vec<Message> {
    let mut messages = vec![Message {
        role: Role::System,
        content: vec![ContentPart::Text {
            text: system_prompt.to_string(),
        }],
    }];
    messages.extend(derived);
    messages
}
