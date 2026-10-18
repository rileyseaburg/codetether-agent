//! Original transcript-policy regression cases.
use codetether_provider_core::{ContentPart, Message, Role};
mod access;
mod denials;
mod examples;
mod false_positives;
mod permissions;
mod phrasing;
mod source_layout;
mod source_preference;
mod temporary;

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
