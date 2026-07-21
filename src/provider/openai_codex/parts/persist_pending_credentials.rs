impl OpenAiCodexProvider {
    async fn persist_pending_credentials(&self, state: &mut OAuthCredentialState) -> Result<()> {
        let Some(previous_refresh) = state.pending_refresh_token.as_deref() else {
            return Ok(());
        };
        let store = self
            .credential_store
            .as_ref()
            .context("Vault credential store is unavailable")?;
        store
            .persist(previous_refresh, &state.credentials)
            .await
            .context("Failed to persist refreshed Codex credentials")?;
        state.pending_refresh_token = None;
        Ok(())
    }
}
