//! Plain-message compatibility during image-input normalization.

#[tokio::test]
async fn message_text_is_unchanged() {
    let prepared = super::prepare(Some("  original text\n".into()), None)
        .await
        .unwrap();
    assert_eq!(prepared.message, "  original text\n");
    assert!(prepared.images.is_empty());
}
