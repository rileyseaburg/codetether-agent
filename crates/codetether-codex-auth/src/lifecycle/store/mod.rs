//! Vault-backed credential rotation and secret conversion.
use super::*;
use codetether_secrets::ProviderSecrets;
use serde_json::{Value, json};
mod load;
mod parse;
mod update;
pub use parse::credentials;
pub use update::updated_vault_secret;
/// Vault-backed OAuth record identified by its provider key.
///
/// # Examples
/// ```
/// let store = codetether_codex_auth::lifecycle::VaultCredentialStore::new("openai-codex");
/// assert_eq!(store.provider_id, "openai-codex");
/// ```
#[derive(Clone)]
pub struct VaultCredentialStore {
    /// Provider key used in the shared secrets service.
    pub provider_id: String,
}

impl VaultCredentialStore {
    /// Select a provider record without reading Vault.
    pub fn new(provider_id: &str) -> Self {
        Self {
            provider_id: provider_id.to_string(),
        }
    }

    ///
    /// # Arguments
    /// * `previous_refresh` - Expected stored refresh token.
    /// * `credentials` - New credential fields.
    ///
    /// # Returns
    /// Persists credentials only when the stored refresh token still matches.
    ///
    /// # Errors
    /// Returns an error for missing Vault records, a changed token, or a failed write.
    ///
    /// # Examples
    /// See [CredentialManager] and [VaultCredentialStore] for configuration.
    pub async fn persist(
        &self,
        previous_refresh: &str,
        credentials: &OAuthCredentials,
    ) -> Result<()> {
        let manager = codetether_secrets::secrets_manager().context("Vault is not configured")?;
        let current = manager
            .get_provider_secrets(&self.provider_id)
            .await?
            .context("Codex credentials are missing from Vault")?;
        let stored_refresh = current
            .extra
            .get("refresh_token")
            .and_then(Value::as_str)
            .context("Codex refresh token is missing from Vault")?;
        if stored_refresh != previous_refresh {
            anyhow::bail!("Codex refresh token changed concurrently in Vault");
        }
        let updated = updated_vault_secret(current, credentials);
        manager
            .set_provider_secrets(&self.provider_id, &updated)
            .await?;
        tracing::info!(provider = %self.provider_id, "Persisted refreshed Codex credentials");
        Ok(())
    }
}
