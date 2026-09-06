//! Text-only transport makes the unsupported modality explicit.

use crate::provider::ContentPart;

#[test]
fn unsupported_image_is_not_silently_dropped_or_flattened_into_base64() {
    let part = ContentPart::Image {
        url: "data:image/png;base64,AQID".into(),
        mime_type: Some("image/png".into()),
    };
    let text = super::render(&part).expect("visible unsupported-image notice");
    assert!(text.contains("does not support image inputs"));
    assert!(!text.contains("AQID"));
}
