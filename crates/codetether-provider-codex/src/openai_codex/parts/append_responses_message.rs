impl OpenAiCodexProvider {
    #[cfg(test)]
    fn append_responses_message(
        message: &Message,
        input: &mut Vec<Value>,
        known_calls: &mut std::collections::HashSet<String>,
    ) {
        codetether_codex_protocol::CodexProtocol::append_responses_message(
            message,
            input,
            known_calls,
        )
    }
}
