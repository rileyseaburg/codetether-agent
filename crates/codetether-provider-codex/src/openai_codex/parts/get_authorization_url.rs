impl OpenAiCodexProvider {
    /// Builds an OAuth authorization URL and its PKCE verifier and state value.
    ///
    /// # Returns
    ///
    /// Returns `(authorization_url, pkce_verifier, state)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use codetether_provider_codex::OpenAiCodexProvider;
    /// let (url, verifier, state) = OpenAiCodexProvider::get_authorization_url();
    /// assert!(url.starts_with("https://auth.openai.com/oauth/authorize"));
    /// assert!(!verifier.is_empty() && !state.is_empty());
    /// ```
    pub fn get_authorization_url() -> (String, String, String) {
        codetether_codex_auth::CodexOAuth::get_authorization_url()
    }
}
