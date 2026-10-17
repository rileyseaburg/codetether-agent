//! text policy.
use super::*;

pub(super) fn text(s: &str) -> Message {
    Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text {
            text: s.to_string(),
        }],
    }
}
