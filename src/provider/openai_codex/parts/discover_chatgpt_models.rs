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
        let url =
            format!("{CHATGPT_CODEX_API_URL}/models?client_version={CHATGPT_CODEX_CLIENT_VERSION}");
        let response = self
            .client
            .get(url)
            .header("Authorization", format!("Bearer {}", auth.access_token))
            .header("chatgpt-account-id", &auth.account_id)
            .header("version", CHATGPT_CODEX_CLIENT_VERSION)
            .header("originator", "codex_cli_rs")
            .send()
            .await
            .context("Failed to query ChatGPT Codex /models")?;
        let status = response.status();
        let body = response.bytes().await?;
        if !status.is_success() {
            anyhow::bail!("ChatGPT Codex /models returned HTTP {status}");
        }
        let models = model_discovery::parse_models(&body)?;
        let slugs: Vec<String> = models
            .iter()
            .map(|m| format!("{}({})", m.slug, m.visibility.as_deref().unwrap_or("-")))
            .collect();
        tracing::info!(
            provider = "openai-codex",
            count = models.len(),
            models = %slugs.join(","),
            "Discovered Codex models"
        );
        model_discovery::store(models.clone());
        Ok(models)
    }
}
