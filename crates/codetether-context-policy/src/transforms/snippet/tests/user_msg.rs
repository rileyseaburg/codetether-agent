//! user msg.
use super::*;

pub(super) fn user_msg(text: &str) -> Message {
    Message {
        role: codetether_provider_core::Role::User,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}
