//! Global secrets write helpers.

use super::SECRETS_MANAGER;
use crate::ProviderSecrets;
use anyhow::Result;

/// Set full provider secrets (convenience function)
///
/// # Errors
/// Returns initialization errors or errors from the registered manager.
///
/// # Examples
/// ```no_run
/// use codetether_secrets::{set_provider_secrets, ProviderSecrets};
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// codetether_secrets::init_from_manager(codetether_secrets::SecretsManager::default()).unwrap();
/// assert!(set_provider_secrets("provider", &ProviderSecrets::default()).await.is_err());
/// # });
/// ```
pub async fn set_provider_secrets(provider_id: &str, secrets: &ProviderSecrets) -> Result<()> {
    match SECRETS_MANAGER.get() {
        Some(manager) => manager.set_provider_secrets(provider_id, secrets).await,
        None => anyhow::bail!("Secrets manager not initialized"),
    }
}

/// Verify the configured Vault backend is reachable (convenience function).
///
/// # Errors
/// Returns initialization errors or errors from the registered manager.
///
/// # Examples
/// ```no_run
/// use codetether_secrets::{verify_reachable};
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// codetether_secrets::init_from_manager(codetether_secrets::SecretsManager::default()).unwrap();
/// assert!(verify_reachable().await.is_err());
/// # });
/// ```
pub async fn verify_reachable() -> Result<()> {
    match SECRETS_MANAGER.get() {
        Some(manager) => manager.verify_reachable().await,
        None => anyhow::bail!("Secrets manager not initialized"),
    }
}
