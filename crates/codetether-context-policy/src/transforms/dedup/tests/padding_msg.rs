//! padding msg.
use super::*;

pub(super) fn padding_msg() -> Message {
    Message {
        role: codetether_provider_core::Role::User,
        content: vec![ContentPart::Text { text: "...".into() }],
    }
}
