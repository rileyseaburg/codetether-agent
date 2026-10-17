//! Secrets manager provider.

use crate::SecretsManager;
use crate::{ProviderSecrets, retry};
use anyhow::Result;
impl SecretsManager {
    /// Get all secrets for a provider
    ///
    /// # Errors
    /// Returns an error if Kubernetes reauthentication fails; ordinary read failures return `Ok(None)`.
    ///
    /// # Examples
    /// ```
    /// use codetether_secrets::{SecretsManager};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let manager = SecretsManager::default();
    /// assert!(manager.get_provider_secrets("provider").await.unwrap().is_none());
    /// # });
    /// ```
    pub async fn get_provider_secrets(&self, provider_id: &str) -> Result<Option<ProviderSecrets>> {
        let client = match self.client() {
            Some(c) => c,
            None => return Ok(None),
        };

        let secret_path = format!("{}/{}", self.path, provider_id);

        let mut result =
            retry::read_secret_with_retry(client.as_ref(), &self.mount, &secret_path).await;
        if matches!(result.as_ref().err(), Some(err) if Self::should_refresh_vault_token(err)) {
            if let Some(client) = self.refresh_kubernetes_auth().await? {
                result =
                    retry::read_secret_with_retry(client.as_ref(), &self.mount, &secret_path).await;
            }
        }

        match result {
            Ok(secret) => Ok(Some(secret)),
            Err(vaultrs::error::ClientError::APIError { code: 404, .. }) => Ok(None),
            Err(e) => {
                tracing::warn!("Failed to fetch secrets for {}: {}", provider_id, e);
                Ok(None)
            }
        }
    }
}
