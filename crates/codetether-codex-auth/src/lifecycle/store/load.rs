//! Load the latest shared credential record.
use super::*;
impl VaultCredentialStore {
    /// Read the current Codex OAuth credentials directly from Vault.
    ///
    /// # Arguments
    /// Uses this store's provider key.
    ///
    /// # Returns
    /// Parsed credentials, or none when the record has no complete OAuth fields.
    ///
    /// # Errors
    /// Returns errors from the shared secrets service.
    ///
    /// # Examples
    /// See [CredentialManager] and [VaultCredentialStore] for configuration.
    pub async fn load(&self) -> Result<Option<OAuthCredentials>> {
        let manager = codetether_secrets::secrets_manager().context("Vault is not configured")?;
        let secrets = manager.get_provider_secrets(&self.provider_id).await?;
        Ok(secrets.as_ref().and_then(credentials))
    }
}
