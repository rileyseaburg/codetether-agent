impl OpenAiCodexProvider {
    fn oauth_expiry(expires_in: u64) -> Result<u64> {
        codetether_codex_auth::CodexOAuth::oauth_expiry(expires_in)
    }
}
