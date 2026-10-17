//! Secrets manager client.

use crate::SecretsManager;
use std::sync::Arc;
use vaultrs::client::VaultClient;
impl SecretsManager {
    /// Check if Vault is configured and connected
    ///
    /// # Examples
    /// ```
    /// use codetether_secrets::{SecretsManager};
    /// let manager = SecretsManager::default();
    /// assert!(!manager.is_connected());
    /// ```
    pub fn is_connected(&self) -> bool {
        self.client.read().is_some()
    }

    pub(crate) fn client(&self) -> Option<Arc<VaultClient>> {
        self.client.read().clone()
    }

    pub(crate) fn with_renewal(self) -> Self {
        self.renewal.start(Arc::downgrade(&self.client));
        self
    }
}
