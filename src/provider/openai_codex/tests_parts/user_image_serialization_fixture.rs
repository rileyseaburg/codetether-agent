const USER_IMAGE_DATA_URL: &str = concat!(
    "data:image/png;base64,",
    "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO+aN1cAAAAASUVORK5CYII="
);
const USER_IMAGE_REMOTE_URL: &str = "https://example.com/screenshot.png";

fn user_image_part(url: &str) -> ContentPart {
    ContentPart::Image {
        url: url.into(),
        mime_type: Some("image/png".into()),
    }
}

fn serialized_user_image_message(message: Message, responses: bool) -> Value {
    let wire = if responses {
        json!(OpenAiCodexProvider::convert_messages_to_responses_input(&[
            message
        ]))
    } else {
        json!(OpenAiCodexProvider::convert_messages(&[message]))
    };
    let bytes = serde_json::to_vec(&wire).unwrap();
    let decoded: Vec<Value> = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(decoded.len(), 1);
    decoded.into_iter().next().unwrap()
}
