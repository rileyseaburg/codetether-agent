impl OpenAiCodexProvider {
    /// Returns the trusted OAuth issuer URL.
    ///
    /// # Examples
    ///
    /// ```
    /// use codetether_provider_codex::OpenAiCodexProvider;
    /// assert_eq!(OpenAiCodexProvider::oauth_issuer(), "https://auth.openai.com");
    /// ```
    pub fn oauth_issuer() -> &'static str {
        codetether_codex_auth::CodexOAuth::oauth_issuer()
    }
}
