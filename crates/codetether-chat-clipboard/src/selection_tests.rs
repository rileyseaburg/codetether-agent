//! Mocked clipboard policy keeps native Windows images ahead of accompanying text.
use super::selection::{Content, read};
use codetether_image::ImageAttachment;

fn image() -> Option<ImageAttachment> {
    Some(ImageAttachment {
        data_url: "data:image/png;base64,cGl4ZWxz".into(),
        mime_type: Some("image/png".into()),
    })
}

#[test]
fn windows_image_wins_without_reading_accompanying_text() {
    let content = read(true, || panic!("image already selected"), image).unwrap();
    assert!(
        matches!(content, Content::Image(image) if image.mime_type.as_deref() == Some("image/png"))
    );
}

#[test]
fn windows_plain_text_remains_available_without_an_image() {
    let content = read(true, || Some("one\r\ntwo".into()), || None).unwrap();
    assert!(matches!(content, Content::Text(text) if text == "one\r\ntwo"));
}

#[test]
fn other_platforms_keep_text_first_policy() {
    let content = read(
        false,
        || Some("text".into()),
        || panic!("text already selected"),
    )
    .unwrap();
    assert!(matches!(content, Content::Text(text) if text == "text"));
    assert!(matches!(
        read(false, || None, image),
        Some(Content::Image(_))
    ));
}

#[test]
fn empty_clipboard_has_no_content() {
    assert!(read(true, || None, || None).is_none());
    assert!(read(false, || None, || None).is_none());
}
