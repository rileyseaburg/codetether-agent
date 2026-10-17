impl OpenAiCodexProvider {
    /// Exchanges an OAuth authorization code using an explicit redirect URI.
    ///
    /// # Arguments
    ///
    /// * `code` — Authorization code returned by the OAuth callback.
    /// * `verifier` — PKCE verifier used to create the authorization request.
    /// * `redirect_uri` — Redirect URI registered for the OAuth client.
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
    /// use codetether_provider_codex::OpenAiCodexProvider;
    /// let credentials = OpenAiCodexProvider::exchange_code_with_redirect_uri(
    ///     "code",
    ///     "verifier",
    ///     "http://localhost:1455/auth/callback",
    /// ).await?;
    /// assert!(!credentials.refresh_token.is_empty());
    /// # Ok::<(), anyhow::Error>(())
    /// # }).unwrap();
    /// ```
    pub async fn exchange_code_with_redirect_uri(
        code: &str,
        verifier: &str,
        redirect_uri: &str,
    ) -> Result<OAuthCredentials> {
        codetether_codex_auth::CodexOAuth::exchange_code_with_redirect_uri(
            code,
            verifier,
            redirect_uri,
        )
        .await
    }
}
