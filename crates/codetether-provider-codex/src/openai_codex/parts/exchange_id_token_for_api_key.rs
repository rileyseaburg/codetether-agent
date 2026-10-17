impl OpenAiCodexProvider {
    /// Exchanges an OpenID identity token for an OpenAI API access token.
    ///
    /// # Arguments
    ///
    /// * `id_token` — Identity token issued by the OpenAI OAuth service.
    ///
    /// # Errors
    ///
    /// Returns an error when the request fails, the exchange is rejected, or
    /// the response contains no usable access token.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_provider_codex::OpenAiCodexProvider;
    /// let access_token = OpenAiCodexProvider::exchange_id_token_for_api_key("id-token").await?;
    /// assert!(!access_token.is_empty());
    /// # Ok::<(), anyhow::Error>(())
    /// # }).unwrap();
    /// ```
    pub async fn exchange_id_token_for_api_key(id_token: &str) -> Result<String> {
        codetether_codex_auth::CodexOAuth::exchange_id_token_for_api_key(id_token).await
    }
}
