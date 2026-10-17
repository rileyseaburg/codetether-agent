//! asst.
use super::*;

pub(super) fn asst(t: &str) -> Message {
    Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text { text: t.into() }],
    }
}
