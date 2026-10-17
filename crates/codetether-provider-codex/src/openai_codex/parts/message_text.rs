impl OpenAiCodexProvider {
    fn message_text(message: &Message, separator: &str) -> String {
        codetether_codex_protocol::CodexProtocol::message_text(message, separator)
    }
}
