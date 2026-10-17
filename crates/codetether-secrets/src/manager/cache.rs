//! Secrets manager cache.

use crate::SecretsManager;

impl SecretsManager {
    /// Check if a provider has an API key in Vault
    ///
    /// # Examples
    /// ```
    /// use codetether_secrets::{SecretsManager};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let manager = SecretsManager::default();
    /// assert!(!manager.has_api_key("provider").await);
    /// # });
    /// ```
    pub async fn has_api_key(&self, provider_id: &str) -> bool {
        matches!(self.get_api_key(provider_id).await, Ok(Some(_)))
    }

    /// Clear the cache (useful when secrets are rotated)
    ///
    /// # Examples
    /// ```
    /// use codetether_secrets::{SecretsManager};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let manager = SecretsManager::default();
    /// manager.cache.write().await.insert("example".into(), String::new());
    /// manager.clear_cache().await;
    /// assert!(manager.cache.read().await.is_empty());
    /// # });
    /// ```
    pub async fn clear_cache(&self) {
        let mut cache = self.cache.write().await;
        cache.clear();
    }
}
