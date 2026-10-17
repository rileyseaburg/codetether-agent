//! Remote image references remain unchanged in the durable child payload.
use super::super::item::InputItem;
use serde_json::json;

#[tokio::test]
async fn forwards_remote_images_without_fetching_or_rewriting() {
    for url in [
        "https://example.invalid/image.png?signature=AbC%2Fxyz&expires=1",
        "http://example.invalid:8080/image",
        "HTTPS://EXAMPLE.invalid:443/image.png#fragment",
    ] {
        let items = vec![InputItem::Image {
            image_url: url.into(),
        }];
        let prepared = super::prepare(None, Some(items)).await.unwrap();
        assert_eq!(prepared.message, "[Image attached]");
        assert_eq!(prepared.images.len(), 1);
        assert_eq!(prepared.images[0].data_url, url);
        assert!(prepared.images[0].mime_type.is_none());
        let payload = json!({"__ct_message_images": prepared.images});
        assert_eq!(payload["__ct_message_images"][0]["data_url"], url);
        assert!(payload["__ct_message_images"][0]["mime_type"].is_null());
    }
}
