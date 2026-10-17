impl OpenAiCodexProvider {
    #[allow(dead_code)]
    fn convert_messages(messages: &[Message]) -> Vec<Value> {
        codetether_codex_protocol::CodexProtocol::convert_messages(messages)
    }
}
