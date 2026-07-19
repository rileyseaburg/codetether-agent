//! Provider-message construction for remote turn evidence.

use crate::provider::{ContentPart, Message, Role};

pub(super) fn text(role: Role, text: &str) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text {
            text: text.to_string(),
        }],
    }
}
