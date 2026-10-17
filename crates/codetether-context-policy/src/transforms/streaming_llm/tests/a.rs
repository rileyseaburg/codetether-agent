//! a.
use super::*;

pub(super) fn a(t: &str) -> Message {
    Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text { text: t.into() }],
    }
}
