impl OpenAiCodexProvider {
    /// Spend the current refresh token and persist the rotated credentials.
    #[cfg(test)]
    async fn rotate_credentials(
        &self,
        state: &mut OAuthCredentialState,
    ) -> Result<OAuthCredentials> {
        self.credential_manager().rotate_credentials(state).await
    }
}
#[cfg(test)]
use codetether_codex_auth::lifecycle::merge_refreshed_credentials;
