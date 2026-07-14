//! Provider-message assembly with optional local recall evidence.

use crate::provider::{ContentPart, Message, Role};

pub(super) fn build(
    system_prompt: &str,
    prefetch: Option<Message>,
    derived: Vec<Message>,
) -> Vec<Message> {
    let mut messages = vec![Message {
        role: Role::System,
        content: vec![ContentPart::Text {
            text: system_prompt.to_string(),
        }],
    }];
    messages.extend(prefetch);
    messages.extend(derived);
    messages
}
