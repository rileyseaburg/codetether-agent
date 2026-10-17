//! Secrets manager reachable.

use crate::SecretsManager;
use anyhow::{Context, Result};
use vaultrs::kv2;
impl SecretsManager {
    /// Verify the configured Vault client can reach the backing server.
    ///
    /// A missing provider path (404) still proves Vault is reachable; transport,
    /// authentication, and permission errors are returned to the caller so CLI
    /// flows can fail before collecting credentials they cannot persist.
    ///
    /// # Errors
    /// Returns an error if no client is configured, authentication refresh fails, or Vault rejects the probe (except for a missing path).
    ///
    /// # Examples
    /// ```
    /// use codetether_secrets::{SecretsManager};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let manager = SecretsManager::default();
    /// assert!(manager.verify_reachable().await.is_err());
    /// # });
    /// ```
    pub async fn verify_reachable(&self) -> Result<()> {
        let client = match self.client() {
            Some(c) => c,
            None => anyhow::bail!("Vault client not configured"),
        };

        let mut result = kv2::list(client.as_ref(), &self.mount, &self.path).await;
        if matches!(result.as_ref().err(), Some(err) if Self::should_refresh_vault_token(err)) {
            if let Some(client) = self.refresh_kubernetes_auth().await? {
                result = kv2::list(client.as_ref(), &self.mount, &self.path).await;
            }
        }

        match result {
            Ok(_) | Err(vaultrs::error::ClientError::APIError { code: 404, .. }) => Ok(()),
            Err(error) => Err(error).context("Failed to reach configured Vault provider path"),
        }
    }
}
