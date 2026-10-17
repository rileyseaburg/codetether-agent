impl OpenAiCodexProvider {
    fn extract_responses_instructions(messages: &[Message]) -> String {
        codetether_codex_protocol::CodexProtocol::extract_responses_instructions(messages)
    }
}
