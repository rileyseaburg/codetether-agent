impl OpenAiCodexProvider {
    #[cfg(test)]
    fn convert_messages_to_responses_input(messages: &[Message]) -> Vec<Value> {
        codetether_codex_protocol::CodexProtocol::convert_messages_to_responses_input(messages)
    }
}
