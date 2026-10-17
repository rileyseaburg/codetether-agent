impl OpenAiCodexProvider {
    /// Spend the current refresh token and persist the rotated credentials.
    async fn rotate_credentials(
        &self,
        state: &mut OAuthCredentialState,
    ) -> Result<OAuthCredentials> {
        self.credential_manager().rotate_credentials(state).await
    }
}
use codetether_codex_auth::lifecycle::merge_refreshed_credentials;
