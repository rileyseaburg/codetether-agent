impl OpenAiCodexProvider {
    /// Adopt credentials another process already rotated into Vault.
    ///
    /// Returns `true` when local state was replaced. Skipped while a local
    /// rotation is still unpersisted so newer local tokens are never dropped.
    async fn sync_shared_credentials(
        store: &VaultCredentialStore,
        state: &mut OAuthCredentialState,
    ) -> bool {
        codetether_codex_auth::lifecycle::CredentialManager::sync_shared_credentials(store, state)
            .await
    }
}
use codetether_codex_auth::lifecycle::adopt_shared_credentials;
