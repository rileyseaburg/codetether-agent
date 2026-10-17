//! Data image preparation and lossless durable serialization.
use super::super::item::InputItem;
use serde_json::json;

#[tokio::test]
async fn image_only_input_retains_durable_attachment() {
    let url = "data:image/png;base64,AA==";
    let prepared = super::prepare(
        None,
        Some(vec![InputItem::Image {
            image_url: url.into(),
        }]),
    )
    .await
    .unwrap();
    assert_eq!(prepared.message, "[Image attached]");
    assert_eq!(prepared.images.len(), 1);
    let payload = json!({"__ct_message_images": prepared.images});
    assert_eq!(payload["__ct_message_images"][0]["data_url"], url);
    assert_eq!(payload["__ct_message_images"][0]["mime_type"], "image/png");
}

#[tokio::test]
async fn invalid_images_fail_even_with_text() {
    for image_url in [
        "ftp://example.invalid/image.png",
        "data:image/png;base64,???",
        "data:text/plain;base64,AA==",
    ] {
        let result = super::prepare(
            None,
            Some(vec![
                InputItem::Text {
                    text: "inspect".into(),
                },
                InputItem::Image {
                    image_url: image_url.into(),
                },
            ]),
        )
        .await;
        assert!(
            result.is_err(),
            "must not silently keep only text: {image_url}"
        );
    }
}
