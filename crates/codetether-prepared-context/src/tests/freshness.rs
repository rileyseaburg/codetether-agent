//! Tests for append-compatible proactive context freshness.

use crate::{fingerprint, matches};
use codetether_provider_core::{ContentPart, Message, Role};

#[test]
fn appended_messages_preserve_prepared_prefix() {
    let mut messages = vec![message("stable")];
    let expected = fingerprint(&messages);
    messages.push(message("new"));
    assert!(matches(&messages, 1, expected));
}

#[test]
fn rewritten_prefix_invalidates_prepared_context() {
    let expected = fingerprint(&[message("old")]);
    assert!(!matches(&[message("changed")], 1, expected));
}

fn message(text: &str) -> Message {
    Message {
        role: Role::User,
        content: vec![ContentPart::Text { text: text.into() }],
    }
}
