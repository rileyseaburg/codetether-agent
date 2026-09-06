include!("user_image_content.rs");

impl OpenAiCodexProvider {
    fn append_responses_user(message: &Message, input: &mut Vec<Value>) {
        let content = if Self::user_has_images(message) {
            Self::user_image_content(message, UserImageFormat::Responses)
        } else {
            let text = Self::message_text(message, "\n");
            if text.is_empty() {
                return;
            }
            vec![json!({ "type": "input_text", "text": text })]
        };
        input.push(json!({
            "type": "message",
            "role": "user",
            "content": content,
        }));
    }
}
