//! Message fixtures for context selection regressions.
use super::*;

pub(super) fn user(text: &str) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text {
            text: text.to_string(),
        }],
    }
}

pub(super) fn assistant(text: &str) -> Message {
    Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text {
            text: text.to_string(),
        }],
    }
}
