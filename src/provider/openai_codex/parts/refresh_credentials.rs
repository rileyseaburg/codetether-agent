impl OpenAiCodexProvider {
    /// Return usable credentials, rotating them at most once across processes.
    ///
    /// Vault-backed providers take a host-wide [`RefreshLock`], re-read Vault,
    /// and only hit the token endpoint when no peer has already rotated.
    async fn refresh_credentials(&self, rejected: Option<&str>) -> Result<OAuthCredentials> {
        let slot = self
            .stored_credentials
            .as_ref()
            .context("No OAuth credentials available. Run OAuth flow first.")?;
        let mut state = slot.write().await;
        self.persist_pending_credentials(&mut state).await?;
        if !refresh_required(&state.credentials, Self::oauth_expiry(0)?, rejected) {
            return Ok(state.credentials.clone());
        }
        let Some(store) = self.credential_store.as_ref() else {
            return self.rotate_credentials(&mut state).await;
        };
        let _lock = RefreshLock::acquire_for(&store.provider_id).await;
        Self::sync_shared_credentials(store, &mut state).await;
        if !refresh_required(&state.credentials, Self::oauth_expiry(0)?, rejected) {
            return Ok(state.credentials.clone());
        }
        match self.rotate_credentials(&mut state).await {
            Ok(credentials) => Ok(credentials),
            Err(error) if Self::sync_shared_credentials(store, &mut state).await => {
                tracing::warn!(%error, "Codex refresh lost a race; using peer-rotated credentials");
                Ok(state.credentials.clone())
            }
            Err(error) => Err(error),
        }
    }
}
