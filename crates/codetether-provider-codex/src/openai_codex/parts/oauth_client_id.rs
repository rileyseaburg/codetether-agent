impl OpenAiCodexProvider {
    /// Returns the public OAuth client identifier used by the Codex flow.
    ///
    /// # Examples
    ///
    /// ```
    /// use codetether_provider_codex::OpenAiCodexProvider;
    /// assert!(!OpenAiCodexProvider::oauth_client_id().is_empty());
    /// ```
    pub fn oauth_client_id() -> &'static str {
        codetether_codex_auth::CodexOAuth::oauth_client_id()
    }
}
