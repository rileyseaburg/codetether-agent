//! Serialization regression tests for native Converse image blocks.
use super::super::image_test_support::*;
use crate::provider::{ContentPart, Message, Role};

#[test]
fn user_images_merge_and_unsupported_references_are_visible() {
    let out = convert(&[
        Message {
            role: Role::User,
            content: vec![image("YQ==")],
        },
        Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "caption".into(),
            }],
        },
    ]);
    assert_eq!(out.as_array().unwrap().len(), 1);
    assert_eq!(out[0]["content"][0]["image"]["source"]["bytes"], "YQ==");
    assert_eq!(out[0]["content"][1]["text"], "caption");
    for url in [
        "https://example.com/a.png",
        "file:///private.png",
        "data:image/png;base64,%%%",
        "data:image/svg+xml;base64,YQ==",
    ] {
        assert!(
            super::super::image::block(url, None)["text"]
                .as_str()
                .unwrap()
                .contains("Image unavailable")
        );
    }
}
