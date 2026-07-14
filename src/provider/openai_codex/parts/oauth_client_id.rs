impl OpenAiCodexProvider {
    /// Returns the public OAuth client identifier used by the Codex flow.
    ///
    /// # Examples
    ///
    /// ```
    /// use codetether_agent::provider::openai_codex::OpenAiCodexProvider;
    /// assert!(!OpenAiCodexProvider::oauth_client_id().is_empty());
    /// ```
    pub fn oauth_client_id() -> &'static str {
        CLIENT_ID
    }
}
