//! Provider-message construction for remote turn evidence.

use codetether_provider_core::{ContentPart, Message, Role};

pub(super) fn text(role: Role, text: &str) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text {
            text: text.to_string(),
        }],
    }
}
