impl OpenAiCodexProvider {
    fn generate_pkce() -> PkcePair {
        codetether_codex_auth::CodexOAuth::generate_pkce()
    }
}
