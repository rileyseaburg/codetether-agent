impl OpenAiCodexProvider {
    async fn force_refresh_access_token(&self) -> Result<String> {
        let stored = self
            .stored_credentials
            .as_ref()
            .context("No OAuth credentials available. Run OAuth flow first.")?;
        let credentials = self.refresh_access_token(&stored.refresh_token).await?;
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
