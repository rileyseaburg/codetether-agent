use super::{input, item::InputItem};

#[tokio::test]
async fn accepts_text_and_image_items() {
    let prepared = input::prepare(
        None,
        Some(vec![
            InputItem::Text {
                text: "inspect this".into(),
            },
            InputItem::Image {
                image_url: "data:image/png;base64,AA==".into(),
            },
        ]),
    )
    .await
    .unwrap();
    assert_eq!(prepared.message, "inspect this\n[Image attached]");
    assert_eq!(prepared.images.len(), 1);
}

#[tokio::test]
async fn requires_exactly_one_input_form() {
    let error = input::prepare(Some("text".into()), Some(Vec::new()))
        .await
        .err()
        .unwrap();
    assert!(error.to_string().contains("either message or items"));
}

#[tokio::test]
async fn rejects_audio_without_provider_support() {
    let error = input::prepare(
        None,
        Some(vec![InputItem::Audio {
            audio_url: "data:audio/wav;base64,AA==".into(),
        }]),
    )
    .await
    .err()
    .unwrap();
    assert!(error.to_string().contains("not supported"));
}
