impl OpenAiCodexProvider {
    /// Exchanges an OAuth authorization code using the default redirect URI.
    ///
    /// # Arguments
    ///
    /// * `code` — Authorization code returned by the OAuth callback.
    /// * `verifier` — PKCE verifier returned by [`Self::get_authorization_url`].
    ///
    /// # Errors
    ///
    /// Returns an error for transport failures, rejected codes, malformed token
    /// responses, or an invalid system clock.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_agent::provider::openai_codex::OpenAiCodexProvider;
    /// let credentials = OpenAiCodexProvider::exchange_code("code", "verifier").await?;
    /// assert!(!credentials.access_token.is_empty());
    /// # Ok::<(), anyhow::Error>(())
    /// # }).unwrap();
    /// ```
    pub async fn exchange_code(code: &str, verifier: &str) -> Result<OAuthCredentials> {
        Self::exchange_code_with_redirect_uri(code, verifier, REDIRECT_URI).await
    }
}
