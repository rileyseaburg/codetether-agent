impl OpenAiCodexProvider {
    /// Return usable credentials, rotating them at most once across processes.
    ///
    /// Vault-backed providers take a host-wide [`RefreshLock`], re-read Vault,
    /// and only hit the token endpoint when no peer has already rotated.
    async fn refresh_credentials(&self, rejected: Option<&str>) -> Result<OAuthCredentials> {
        self.credential_manager()
            .refresh_credentials(rejected)
            .await
    }
}
