//! Serialization regression tests for native Anthropic image blocks.
use super::super::image_test_support::*;
use crate::provider::{Message, Role};

#[test]
fn orphan_images_do_not_leak_into_known_results() {
    let out = convert(&[
        calls(),
        Message {
            role: Role::Tool,
            content: vec![
                result("a", "plain"),
                result("missing", "orphan"),
                image("Yg=="),
            ],
        },
    ]);
    assert_eq!(out[1]["content"].as_array().unwrap().len(), 1);
    assert_eq!(out[1]["content"][0]["content"], "plain");
}
