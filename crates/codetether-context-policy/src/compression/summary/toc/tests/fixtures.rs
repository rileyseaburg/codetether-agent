//! Message fixtures for dropped-range navigation tests.
use codetether_provider_core::{ContentPart, Message, Role};

pub(super) fn text(role: Role, s: &str) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text {
            text: s.to_string(),
        }],
    }
}
