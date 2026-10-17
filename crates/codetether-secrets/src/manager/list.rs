//! Secrets manager list.

use crate::SecretsManager;
use anyhow::Result;
use vaultrs::kv2;
impl SecretsManager {
    /// List all providers that have secrets configured
    ///
    /// # Errors
    /// Returns an error if Kubernetes reauthentication fails; ordinary listing failures return an empty list.
    ///
    /// # Examples
    /// ```
    /// use codetether_secrets::{SecretsManager};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let manager = SecretsManager::default();
    /// assert!(manager.list_configured_providers().await.unwrap().is_empty());
    /// # });
    /// ```
    pub async fn list_configured_providers(&self) -> Result<Vec<String>> {
        let client = match self.client() {
            Some(c) => c,
            None => return Ok(Vec::new()),
        };

        let mut result = kv2::list(client.as_ref(), &self.mount, &self.path).await;
        if matches!(result.as_ref().err(), Some(err) if Self::should_refresh_vault_token(err)) {
            if let Some(client) = self.refresh_kubernetes_auth().await? {
                result = kv2::list(client.as_ref(), &self.mount, &self.path).await;
            }
        }

        match result {
            Ok(keys) => Ok(keys),
            Err(vaultrs::error::ClientError::APIError { code: 404, .. }) => Ok(Vec::new()),
            Err(e) => {
                tracing::warn!("Failed to list providers: {}", e);
                Ok(Vec::new())
            }
        }
    }
}
