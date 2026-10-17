//! Secrets manager write.

use crate::ProviderSecrets;
use crate::SecretsManager;
use anyhow::{Context, Result};
use vaultrs::kv2;
impl SecretsManager {
    /// Set/update secrets for a provider in Vault
    ///
    /// # Errors
    /// Returns an error if no client exists, Kubernetes reauthentication fails, or Vault cannot persist the record.
    ///
    /// # Examples
    /// ```
    /// use codetether_secrets::{SecretsManager, ProviderSecrets};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let manager = SecretsManager::default();
    /// assert!(manager.set_provider_secrets("provider", &ProviderSecrets::default()).await.is_err());
    /// # });
    /// ```
    pub async fn set_provider_secrets(
        &self,
        provider_id: &str,
        secrets: &ProviderSecrets,
    ) -> Result<()> {
        let client = match self.client() {
            Some(c) => c,
            None => anyhow::bail!("Vault client not configured"),
        };

        let secret_path = format!("{}/{}", self.path, provider_id);
        let mut result = kv2::set(client.as_ref(), &self.mount, &secret_path, secrets).await;
        if matches!(result.as_ref().err(), Some(err) if Self::should_refresh_vault_token(err)) {
            if let Some(client) = self.refresh_kubernetes_auth().await? {
                result = kv2::set(client.as_ref(), &self.mount, &secret_path, secrets).await;
            }
        }
        result.with_context(|| format!("Failed to write provider secrets for {}", provider_id))?;

        // Update cache with latest API key value
        let mut cache = self.cache.write().await;
        if let Some(api_key) = secrets.api_key.clone() {
            cache.insert(provider_id.to_string(), api_key);
        } else {
            cache.remove(provider_id);
        }

        Ok(())
    }
}
