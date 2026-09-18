//! Interactive-profile resolution followed by existing workload/environment auth.

use super::{SecretsManager, VaultConfig};
use anyhow::{Context, Result};

impl SecretsManager {
    /// Load the active CodeTether login, or workload/environment credentials.
    ///
    /// # Errors
    /// Returns an error if settings are absent, invalid, or cannot authenticate.
    /// # Examples
    /// ```no_run
    /// # async fn example() -> anyhow::Result<()> {
    /// let manager = codetether_agent::secrets::SecretsManager::from_env().await?;
    /// assert!(manager.is_connected());
    /// # Ok(()) }
    /// ```
    pub async fn from_env() -> Result<Self> {
        if let Some(config) = super::login::configured()? {
            return Self::new(&config).await;
        }
        let address =
            std::env::var("VAULT_ADDR").context("VAULT_ADDR not set; run codetether vault url")?;
        let mount = std::env::var("VAULT_MOUNT").ok();
        let path = std::env::var("VAULT_SECRETS_PATH").ok();
        if let Some(manager) =
            super::environment_k8s::authenticate(&address, mount.as_deref(), path.as_deref()).await
        {
            return Ok(manager);
        }
        let token = std::env::var("VAULT_TOKEN")
            .context("Vault login is required; run codetether vault login")?;
        Self::new(&VaultConfig {
            address,
            token,
            mount,
            path,
        })
        .await
    }
}
