impl OpenAiCodexProvider {
    async fn current_credentials(&self) -> Result<OAuthCredentials> {
        self.refresh_credentials(None).await
    }
}
