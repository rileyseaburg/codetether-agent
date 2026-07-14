impl OpenAiCodexProvider {
    /// Returns the trusted OAuth issuer URL.
    ///
    /// # Examples
    ///
    /// ```
    /// use codetether_agent::provider::openai_codex::OpenAiCodexProvider;
    /// assert_eq!(OpenAiCodexProvider::oauth_issuer(), "https://auth.openai.com");
    /// ```
    pub fn oauth_issuer() -> &'static str {
        AUTH_ISSUER
    }
}
