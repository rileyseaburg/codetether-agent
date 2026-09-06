//! Serialization regression tests for native Converse image blocks.
use super::super::image_test_support::*;
use crate::provider::{Message, Role};

#[test]
fn orphan_images_are_removed_with_results_and_empty_text_is_preserved() {
    let out = convert(&[
        calls(),
        Message {
            role: Role::Tool,
            content: vec![
                result("a", ""),
                result("missing", "orphan"),
                image("Yg=="),
                result("b", "plain"),
            ],
        },
    ]);
    let parts = &out[1]["content"];
    assert_eq!(parts.as_array().unwrap().len(), 2);
    assert_eq!(
        parts[0]["toolResult"]["content"][0]["text"],
        "(empty tool result)"
    );
    assert_eq!(parts[1]["toolResult"]["content"][0]["text"], "plain");
    assert_eq!(
        parts[0]["toolResult"]["content"].as_array().unwrap().len(),
        1
    );
    assert_eq!(
        parts[1]["toolResult"]["content"].as_array().unwrap().len(),
        1
    );
}
