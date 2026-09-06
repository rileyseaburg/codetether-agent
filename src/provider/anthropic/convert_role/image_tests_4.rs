//! Serialization regression tests for native Anthropic image blocks.
use super::super::image_test_support::*;
use crate::provider::{ContentPart, Message, Role};

#[test]
fn text_only_user_is_unchanged() {
    assert_eq!(
        convert(&[Message {
            role: Role::User,
            content: vec![ContentPart::Text { text: "hi".into() }]
        }])[0]["content"][0]["text"],
        "hi"
    );
}
