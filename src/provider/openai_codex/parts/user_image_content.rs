// Responses shape matches openai/codex e01f38c388f4907f02ac5b4980a37487686204c8,
// codex-rs/protocol/src/models.rs: ContentItem::{InputText, InputImage}.
enum UserImageFormat {
    Responses,
    Chat,
}

impl OpenAiCodexProvider {
    fn user_has_images(message: &Message) -> bool {
        message
            .content
            .iter()
            .any(|part| matches!(part, ContentPart::Image { .. }))
    }

    fn user_image_content(message: &Message, format: UserImageFormat) -> Vec<Value> {
        message
            .content
            .iter()
            .filter_map(|part| match (&format, part) {
                (UserImageFormat::Responses, ContentPart::Text { text }) => {
                    Some(json!({ "type": "input_text", "text": text }))
                }
                (UserImageFormat::Responses, ContentPart::Image { url, .. }) => {
                    Some(json!({ "type": "input_image", "image_url": url }))
                }
                (UserImageFormat::Chat, ContentPart::Text { text }) => {
                    Some(json!({ "type": "text", "text": text }))
                }
                (UserImageFormat::Chat, ContentPart::Image { url, .. }) => {
                    Some(json!({ "type": "image_url", "image_url": { "url": url } }))
                }
                _ => None,
            })
            .collect()
    }
}
