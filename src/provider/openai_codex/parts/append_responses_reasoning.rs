impl OpenAiCodexProvider {
    fn append_responses_reasoning(message: &Message, input: &mut Vec<Value>) {
        for part in &message.content {
            let ContentPart::Thinking {
                signature: Some(signature),
                ..
            } = part
            else {
                continue;
            };
            if let Some(item) = super::codex_reasoning::decode_signature(signature) {
                input.push(item);
            }
        }
    }
}
