//! Vertex user images share the native Anthropic wire shape and notices.
use super::super::fixtures::*;
use crate::provider::Role;
use serde_json::json;

#[test]
fn data_images_keep_order_and_remote_images_report_unsupported_reference() {
    let out = convert(&[message(
        Role::User,
        vec![
            text("caption"),
            image("data:image/png;base64,YQ=="),
            image("https://example.com/image.png"),
        ],
    )]);
    assert_eq!(
        out["messages"][0]["content"],
        json!([
            {"type": "text", "text": "caption"},
            {"type": "image", "source": {"type": "base64", "media_type": "image/png", "data": "YQ=="}},
            {"type": "text", "text": "[Image unavailable: Vertex Anthropic requires base64 image data; remote image URLs are not fetched.]"},
        ])
    );
}
#[test]
fn unsupported_images_emit_notices_instead_of_disappearing() {
    for url in [
        "file:///private.png",
        "data:image/png;base64,%%%",
        "s3://bucket/image.png",
    ] {
        let out = convert(&[message(Role::User, vec![image(url)])]);
        assert!(
            out["messages"][0]["content"][0]["text"]
                .as_str()
                .unwrap()
                .contains("Image unavailable")
        );
    }
}
#[test]
fn image_only_user_turn_is_not_a_blank_placeholder() {
    let out = convert(&[message(
        Role::User,
        vec![image("data:image/png;base64,YQ==")],
    )]);
    assert_eq!(out["messages"][0]["content"][0]["type"], "image");
}
