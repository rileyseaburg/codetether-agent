impl OpenAiCodexProvider {
    #[cfg(test)]
    async fn persist_pending_credentials(&self, state: &mut OAuthCredentialState) -> Result<()> {
        self.credential_manager()
            .persist_pending_credentials(state)
            .await
    }
}
