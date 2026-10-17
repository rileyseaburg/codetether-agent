//! Secrets manager connect.

use crate::SecretsManager;
use crate::VaultConfig;
use anyhow::{Context, Result};
use parking_lot::RwLock as ClientRwLock;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
impl SecretsManager {
    /// Create a new secrets manager with Vault configuration
    ///
    /// # Errors
    /// Returns an error if Vault client settings or the client cannot be constructed.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_secrets::{SecretsManager, VaultConfig};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let config = VaultConfig { address: "http://127.0.0.1:8200".into(),
    ///     ..VaultConfig::default() };
    /// let manager = SecretsManager::new(&config).await.unwrap();
    /// assert!(manager.is_connected());
    /// # });
    /// ```
    pub async fn new(config: &VaultConfig) -> Result<Self> {
        let settings = VaultClientSettingsBuilder::default()
            .address(&config.address)
            .token(&config.token)
            .build()
            .context("Failed to build Vault client settings")?;

        let client = VaultClient::new(settings).context("Failed to create Vault client")?;

        Ok(Self {
            client: Arc::new(ClientRwLock::new(Some(Arc::new(client)))),
            cache: Arc::new(RwLock::new(HashMap::new())),
            mount: config.mount.clone().unwrap_or_else(|| "secret".to_string()),
            path: config
                .path
                .clone()
                .unwrap_or_else(|| "codetether/providers".to_string()),
            k8s_auth: None,
            renewal: Arc::default(),
        }
        .with_renewal())
    }
}
