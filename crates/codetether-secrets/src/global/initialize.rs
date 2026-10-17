//! Global secrets initialize helpers.

use super::SECRETS_MANAGER;
use crate::{SecretsManager, VaultConfig};
use anyhow::Result;

/// Initialize the global secrets manager
///
/// # Errors
/// Returns initialization errors or errors from the registered manager.
///
/// # Examples
/// ```no_run
/// use codetether_secrets::{init_secrets_manager, VaultConfig};
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// let config = VaultConfig { address: "http://127.0.0.1:8200".into(), ..Default::default() };
/// assert!(init_secrets_manager(&config).await.is_ok());
/// # });
/// ```
pub async fn init_secrets_manager(config: &VaultConfig) -> Result<()> {
    let manager = SecretsManager::new(config).await?;
    SECRETS_MANAGER
        .set(manager)
        .map_err(|_| anyhow::anyhow!("Secrets manager already initialized"))?;
    Ok(())
}

/// Initialize the global secrets manager from an existing manager instance
///
/// # Errors
/// Returns initialization errors or errors from the registered manager.
///
/// # Examples
/// ```no_run
/// use codetether_secrets::{init_from_manager, SecretsManager};
/// assert!(init_from_manager(SecretsManager::default()).is_ok());
/// ```
pub fn init_from_manager(manager: SecretsManager) -> Result<()> {
    SECRETS_MANAGER
        .set(manager)
        .map_err(|_| anyhow::anyhow!("Secrets manager already initialized"))?;
    Ok(())
}

/// Get the global secrets manager
///
/// # Examples
/// ```no_run
/// use codetether_secrets::{secrets_manager};
/// codetether_secrets::init_from_manager(codetether_secrets::SecretsManager::default()).unwrap();
/// assert!(secrets_manager().is_some());
/// ```
pub fn secrets_manager() -> Option<&'static SecretsManager> {
    SECRETS_MANAGER.get()
}
