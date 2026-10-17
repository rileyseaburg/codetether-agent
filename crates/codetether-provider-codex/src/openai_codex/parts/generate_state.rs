impl OpenAiCodexProvider {
    fn generate_state() -> String {
        codetether_codex_auth::CodexOAuth::generate_state()
    }
}
