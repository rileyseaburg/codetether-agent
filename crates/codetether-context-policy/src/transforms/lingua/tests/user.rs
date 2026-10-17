//! user.
use super::*;

pub(super) fn user(t: &str) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text { text: t.into() }],
    }
}
