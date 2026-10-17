impl OpenAiCodexProvider {
    async fn force_refresh_access_token(&self, rejected_token: &str) -> Result<String> {
        let credentials = self.refresh_credentials(Some(rejected_token)).await?;
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
        *self.cached_tokens.write().await = Some(cached);
        Ok(token)
    }
}
