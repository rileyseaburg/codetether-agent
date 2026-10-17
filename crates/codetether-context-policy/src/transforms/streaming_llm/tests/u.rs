//! u.
use super::*;

pub(super) fn u(t: &str) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text { text: t.into() }],
    }
}
