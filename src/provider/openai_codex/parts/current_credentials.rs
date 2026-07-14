impl OpenAiCodexProvider {
    async fn current_credentials(&self) -> Result<OAuthCredentials> {
        let stored = self
            .stored_credentials
            .as_ref()
            .context("No OAuth credentials available. Run OAuth flow first.")?;
        let now = Self::oauth_expiry(0)?;
        if stored.expires_at > now + 300 {
            return Ok(stored.clone());
        }
        self.refresh_access_token(&stored.refresh_token).await
    }
}
