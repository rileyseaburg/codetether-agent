//! Strict provider-record reads for credential lifecycle operations.
use crate::{ProviderSecrets, SecretsManager, retry};
use anyhow::{Result, anyhow};

impl SecretsManager {
    /// Read a provider record without treating backend failures as missing data.
    ///
    /// # Arguments
    /// * `provider_id` - Record beneath the configured provider namespace.
    /// # Returns
    /// The record, or `None` only for Vault's missing-record response.
    /// # Errors
    /// Returns a sanitized error for unavailable, rejected or failed reads.
    /// # Examples
    /// ```
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let manager = codetether_secrets::SecretsManager::default();
    /// assert!(manager.read_provider_secrets("monarch").await.is_err());
    /// # });
    /// ```
    pub async fn read_provider_secrets(
        &self,
        provider_id: &str,
    ) -> Result<Option<ProviderSecrets>> {
        let client = self
            .client()
            .ok_or_else(|| anyhow!("Vault client not configured"))?;
        let path = format!("{}/{}", self.path, provider_id);
        let mut result = retry::read_secret_with_retry(client.as_ref(), &self.mount, &path).await;
        if matches!(result.as_ref().err(), Some(error) if Self::should_refresh_vault_token(error)) {
            let refreshed = self
                .refresh_auth(&client)
                .await
                .map_err(|_| anyhow!("Vault authentication refresh failed"))?;
            if let Some(client) = refreshed {
                result = retry::read_secret_with_retry(client.as_ref(), &self.mount, &path).await;
            }
        }
        match result {
            Ok(record) => Ok(Some(record)),
            Err(vaultrs::error::ClientError::APIError { code: 404, .. }) => Ok(None),
            Err(_) => Err(anyhow!("Vault provider record read failed")),
        }
    }
}
