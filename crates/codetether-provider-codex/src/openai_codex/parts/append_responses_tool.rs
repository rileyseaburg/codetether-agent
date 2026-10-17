impl OpenAiCodexProvider {
    fn append_responses_tool(
        message: &Message,
        input: &mut Vec<Value>,
        known_calls: &std::collections::HashSet<String>,
    ) {
        codetether_codex_protocol::CodexProtocol::append_responses_tool(message, input, known_calls)
    }
}
