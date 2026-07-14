impl OpenAiCodexProvider {
    /// Opens an authenticated Responses WebSocket for the configured backend.
    ///
    /// # Errors
    ///
    /// Returns an error when credentials are unavailable, OAuth is disabled,
    /// request headers are invalid, or the WebSocket handshake fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_agent::provider::openai_codex::OpenAiCodexProvider;
    /// let provider = OpenAiCodexProvider::from_api_key("api-key".to_string());
    /// let _connection = provider.connect_responses_ws().await?;
    /// # Ok::<(), anyhow::Error>(())
    /// # }).unwrap();
    /// ```
    pub async fn connect_responses_ws(&self) -> Result<OpenAiRealtimeConnection> {
        let token = self.get_access_token().await?;
        let account_id = self.resolved_chatgpt_account_id(&token);
        let backend = if self.using_chatgpt_backend() {
            ResponsesWsBackend::ChatGptCodex
        } else {
            ResponsesWsBackend::OpenAi
        };
        self.connect_responses_ws_with_token(&token, account_id.as_deref(), backend)
            .await
    }
}
