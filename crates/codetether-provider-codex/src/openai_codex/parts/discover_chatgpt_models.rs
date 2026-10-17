impl OpenAiCodexProvider {
    /// Fetch the account's model catalog from the Codex backend `/models`.
    ///
    /// Uses the same bearer token, account header, and client version as
    /// Codex responses requests, and stores the result in the process-wide
    /// discovery cache.
    ///
    /// # Errors
    ///
    /// Fails when not using the ChatGPT backend, when auth is unavailable,
    /// on HTTP errors, or when the body is not a model catalog.
    async fn discover_chatgpt_models(&self) -> Result<Vec<model_discovery::DiscoveredModel>> {
        if !self.using_chatgpt_backend() {
            anyhow::bail!("model discovery requires the ChatGPT Codex backend");
        }
        if let Some(models) = model_discovery::discovered() {
            return Ok(models);
        }
        let auth = self.chatgpt_backend_auth().await?;
        codetether_codex_transport::discover_chatgpt_models(
            &self.client,
            &auth.access_token,
            &auth.account_id,
        )
        .await
    }
}
