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
    /// use codetether_agent::provider::openai_codex::OpenAiCodexProvider;
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
        let form_body = format!(
            "grant_type={}&client_id={}&code={}&code_verifier={}&redirect_uri={}",
            urlencoding::encode("authorization_code"),
            CLIENT_ID,
            urlencoding::encode(code),
            urlencoding::encode(verifier),
            urlencoding::encode(redirect_uri),
        );
        let response = crate::provider::shared_http::shared_client()
            .post(TOKEN_URL)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(form_body)
            .send()
            .await
            .context("Failed to exchange authorization code")?;
        if !response.status().is_success() {
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("OAuth token exchange failed: {body}");
        }
        let tokens: OAuthTokenResponse = response
            .json()
            .await
            .context("Failed to parse token response")?;
        let expires_at = Self::oauth_expiry(tokens.expires_in)?;
        let chatgpt_account_id = tokens
            .id_token
            .as_deref()
            .and_then(Self::extract_chatgpt_account_id_from_jwt);
        Ok(OAuthCredentials {
            id_token: tokens.id_token,
            chatgpt_account_id,
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
            expires_at,
        })
    }
}
