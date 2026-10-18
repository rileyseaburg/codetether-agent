impl OpenAiCodexProvider {
    /// Returns a stable, privacy-safe routing key for a workspace prompt.
    #[cfg(test)]
    fn prompt_cache_key(instructions: &str) -> String {
        codetether_codex_protocol::CodexProtocol::prompt_cache_key(instructions)
    }
}
