use base64::Engine;

use super::attachment_from_data_url;

#[test]
fn accepts_image_data_url() {
    let payload = base64::engine::general_purpose::STANDARD.encode("png bytes");
    let text = format!("data:image/png;base64,{payload}");
    let image = attachment_from_data_url(&text).expect("image data URL");
    assert_eq!(image.mime_type.as_deref(), Some("image/png"));
    assert_eq!(image.data_url, text);
}

#[test]
fn rejects_non_image_data_url() {
    let payload = base64::engine::general_purpose::STANDARD.encode("hello");
    let text = format!("data:text/plain;base64,{payload}");
    assert!(attachment_from_data_url(&text).is_none());
}

#[test]
fn normalizes_wrapped_payload() {
    let payload = base64::engine::general_purpose::STANDARD.encode("png bytes");
    let wrapped = format!("data:image/png;base64,{}\n{}", &payload[..4], &payload[4..]);
    let image = attachment_from_data_url(&wrapped).expect("wrapped image data URL");
    assert_eq!(image.data_url, format!("data:image/png;base64,{payload}"));
}
