//! Secrets manager key.

use crate::SecretsManager;
use crate::retry;
use anyhow::Result;
impl SecretsManager {
    /// Get an API key for a provider from Vault
    ///
    /// # Errors
    /// Returns an error if Kubernetes reauthentication fails; ordinary read failures return `Ok(None)`.
    ///
    /// # Examples
    /// ```
    /// use codetether_secrets::{SecretsManager};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let manager = SecretsManager::default();
    /// assert!(manager.get_api_key("provider").await.unwrap().is_none());
    /// # });
    /// ```
    pub async fn get_api_key(&self, provider_id: &str) -> Result<Option<String>> {
        // Check cache first
        {
            let cache = self.cache.read().await;
            if let Some(key) = cache.get(provider_id) {
                return Ok(Some(key.clone()));
            }
        }

        // Fetch from Vault
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
            Ok(secret) => {
                // Cache the result
                if let Some(ref api_key) = secret.api_key {
                    let mut cache = self.cache.write().await;
                    cache.insert(provider_id.to_string(), api_key.clone());
                }
                Ok(secret.api_key)
            }
            Err(vaultrs::error::ClientError::APIError { code: 404, .. }) => Ok(None),
            Err(e) => {
                tracing::warn!("Failed to fetch secret for {}: {}", provider_id, e);
                Ok(None)
            }
        }
    }
}
