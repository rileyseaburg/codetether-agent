//! Interactive-profile resolution followed by existing workload/environment auth.

use super::{SecretsManager, VaultConfig};
use crate::token_file::{TokenFileAuth, read_token};
use anyhow::{Context, Result};
use std::sync::Arc;

impl SecretsManager {
    /// Load the active CodeTether login, or workload/environment credentials.
    ///
    /// Resolution order: saved login profile, Kubernetes workload auth,
    /// `VAULT_TOKEN_FILE` (an externally rotated token file, re-read when Vault
    /// rejects the token), then `VAULT_TOKEN`.
    ///
    /// # Errors
    /// Returns an error if settings are absent, invalid, or cannot authenticate,
    /// including an unreadable or empty `VAULT_TOKEN_FILE`.
    /// # Examples
    /// ```no_run
    /// # async fn example() -> anyhow::Result<()> {
    /// let manager = codetether_secrets::SecretsManager::from_env().await?;
    /// assert!(manager.is_connected());
    /// # Ok(()) }
    /// ```
    pub async fn from_env() -> Result<Self> {
        // Explicit workload identity overrides stale desktop tokens, without
        // silently falling back to a different identity on login failure.
        if let Some(auth) = super::approle::configured()? {
            return Self::from_approle(
                auth,
                std::env::var("VAULT_MOUNT").ok(),
                std::env::var("VAULT_SECRETS_PATH").ok(),
            )
            .await;
        }
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
        if let Some(auth) = TokenFileAuth::from_env(&address) {
            return Self::from_token_file(auth, mount, path).await;
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

    /// Connect with a token file that is re-read when Vault rejects the token.
    pub(crate) async fn from_token_file(
        auth: TokenFileAuth,
        mount: Option<String>,
        path: Option<String>,
    ) -> Result<Self> {
        let token = read_token(&auth.path).await?;
        let mut manager = Self::new(&VaultConfig {
            address: auth.address.clone(),
            token,
            mount,
            path,
        })
        .await?;
        tracing::info!(token_file = %auth.path.display(), "Vault token loaded from file");
        manager.token_file = Some(Arc::new(auth));
        Ok(manager)
    }
}
