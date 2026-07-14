impl OpenAiCodexProvider {
    #[allow(dead_code)]
    fn convert_messages(messages: &[Message]) -> Vec<Value> {
        messages.iter().map(Self::convert_message).collect()
    }
}
