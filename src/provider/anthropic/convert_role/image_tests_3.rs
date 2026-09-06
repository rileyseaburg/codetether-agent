//! Serialization regression tests for native Anthropic image blocks.
use super::super::image_test_support::*;
use crate::provider::{ContentPart, Message, Role};

#[test]
fn user_images_and_unsupported_references_are_visible() {
    let out = convert(&[Message {
        role: Role::User,
        content: vec![
            image("YQ=="),
            ContentPart::Image {
                url: "https://example.com/a.png".into(),
                mime_type: None,
            },
            ContentPart::Image {
                url: "file:///private.png".into(),
                mime_type: None,
            },
        ],
    }]);
    assert_eq!(out[0]["content"][0]["source"]["data"], "YQ==");
    assert_eq!(out[0]["content"][1]["source"]["type"], "url");
    assert!(
        out[0]["content"][2]["text"]
            .as_str()
            .unwrap()
            .contains("Image unavailable")
    );
    for url in [
        "data:image/png;base64,%%%",
        "data:image/svg+xml;base64,YQ==",
    ] {
        assert_eq!(super::super::image::block(url, None)["type"], "text");
    }
}
