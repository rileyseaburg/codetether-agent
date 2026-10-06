impl OpenAiCodexProvider {
    /// Adopt credentials another process already rotated into Vault.
    ///
    /// Returns `true` when local state was replaced. Skipped while a local
    /// rotation is still unpersisted so newer local tokens are never dropped.
    async fn sync_shared_credentials(
        store: &VaultCredentialStore,
        state: &mut OAuthCredentialState,
    ) -> bool {
        if state.pending_refresh_token.is_some() {
            return false;
        }
        let shared = match store.load().await {
            Ok(Some(shared)) => shared,
            Ok(None) => return false,
            Err(error) => {
                tracing::warn!(provider = %store.provider_id, %error, "Codex credential sync failed");
                return false;
            }
        };
        let Some(adopted) = adopt_shared_credentials(&state.credentials, shared) else {
            return false;
        };
        tracing::info!(provider = %store.provider_id, "Adopted Codex credentials rotated by a peer");
        state.credentials = adopted;
        true
    }
}

/// Prefer the shared copy when its refresh token has moved past ours.
fn adopt_shared_credentials(
    local: &OAuthCredentials,
    shared: OAuthCredentials,
) -> Option<OAuthCredentials> {
    (shared.refresh_token != local.refresh_token)
        .then(|| merge_refreshed_credentials(local, shared))
}
