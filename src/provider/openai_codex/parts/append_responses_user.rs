impl OpenAiCodexProvider {
    fn append_responses_user(message: &Message, input: &mut Vec<Value>) {
        let text = Self::message_text(message, "\n");
        if !text.is_empty() {
            input.push(json!({
                "type": "message",
                "role": "user",
                "content": [{ "type": "input_text", "text": text }],
            }));
        }
    }
}
