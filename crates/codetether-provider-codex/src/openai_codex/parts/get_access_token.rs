impl OpenAiCodexProvider {
    async fn get_access_token(&self) -> Result<String> {
        if let Some(api_key) = &self.static_api_key {
            return Ok(api_key.clone());
        }
        if !Self::chatgpt_backend_opted_in() {
            anyhow::bail!(
                "OpenAI Codex ChatGPT backend is disabled. Configure OPENAI_API_KEY for the official OpenAI API, or set {CHATGPT_BACKEND_OPT_IN_ENV}=1 to opt in."
            );
        }
        if let Some(token) = self.cached_access_token().await {
            return Ok(token);
        }
        let mut cache = self.cached_tokens.write().await;
        let credentials = self.current_credentials().await?;
        let now = Self::oauth_expiry(0)?;
        let cached = CachedTokens {
            access_token: credentials.access_token.clone(),
            expires_at: token_expiry::cache_deadline(
                credentials.expires_at,
                now,
                std::time::Instant::now(),
            ),
        };
        let token = cached.access_token.clone();
        *cache = Some(cached);
        Ok(token)
    }
}
