//! Mocked metadata conversion retains image arrays without inventing text.

use super::*;
use base64::Engine;
use serde_json::json;

#[test]
fn result_images_accept_single_multiple_and_optional_mime() {
    let first = encoded(&[1, 2, 3], "image/png");
    let second = json!({"data_url": "https://example.com/image.png"});
    for value in [first.clone(), json!([first, second])] {
        let count = if value.is_array() { 2 } else { 1 };
        let metadata = HashMap::from([("image_data_url".into(), value)]);
        let images = content(Some(&metadata));
        assert_eq!(images.len(), count);
        assert!(matches!(&images[0], ContentPart::Image { url, mime_type }
            if url == "data:image/png;base64,AQID" && mime_type.as_deref() == Some("image/png")));
        if count == 2 {
            assert!(matches!(
                &images[1],
                ContentPart::Image {
                    mime_type: None,
                    ..
                }
            ));
        }
    }
}

#[test]
fn result_images_ignore_absent_and_malformed_entries() {
    assert!(content(None).is_empty());
    let metadata = HashMap::from([("image_data_url".into(), json!([null, {}, {"data_url":""}]))]);
    assert!(content(Some(&metadata)).is_empty());
}

#[test]
fn result_images_encoder_roundtrips_bytes_once() {
    let bytes = b"opaque image fixture";
    let value = encoded(bytes, "image/webp");
    let url = value["data_url"].as_str().unwrap();
    let payload = url.strip_prefix("data:image/webp;base64,").unwrap();
    assert_eq!(
        base64::engine::general_purpose::STANDARD
            .decode(payload)
            .unwrap(),
        bytes
    );
}
