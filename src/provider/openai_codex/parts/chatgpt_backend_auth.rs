impl OpenAiCodexProvider {
    /// Resolves a current access token and account for the opted-in backend.
    ///
    /// # Errors
    ///
    /// Returns an error when backend use is not opted in, credentials cannot
    /// be refreshed, or the ChatGPT account ID is unavailable.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_agent::provider::openai_codex::OpenAiCodexProvider;
    /// let provider = OpenAiCodexProvider::new();
    /// let _auth = provider.chatgpt_backend_auth().await;
    /// # });
    /// ```
    pub async fn chatgpt_backend_auth(&self) -> Result<ChatGptBackendAuth> {
        if !Self::chatgpt_backend_opted_in() {
            anyhow::bail!("ChatGPT Codex backend is not opted in");
        }
        let access_token = self.get_access_token().await?;
        let account_id = self
            .resolved_chatgpt_account_id(&access_token)
            .context("ChatGPT account ID unavailable")?;
        Ok(ChatGptBackendAuth {
            access_token,
            account_id,
        })
    }
}
