//! Regression tests for prior-context access directives.

mod calls;
mod delegated_authority;
mod delegation_calls;
mod human_authority;
mod persistence;

use crate::provider::{ContentPart, Message, Role};

fn user(text: &str) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}

fn image_user() -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Image {
            url: "data:image/png;base64,AA==".into(),
            mime_type: Some("image/png".into()),
        }],
    }
}
