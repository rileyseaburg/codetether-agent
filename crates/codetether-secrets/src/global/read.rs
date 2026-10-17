//! Global secrets read helpers.

use super::SECRETS_MANAGER;
use crate::ProviderSecrets;

/// Get API key for a provider (convenience function)
///
/// # Examples
/// ```no_run
/// use codetether_secrets::{get_api_key};
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// codetether_secrets::init_from_manager(codetether_secrets::SecretsManager::default()).unwrap();
/// assert!(get_api_key("provider").await.is_none());
/// # });
/// ```
pub async fn get_api_key(provider_id: &str) -> Option<String> {
    match SECRETS_MANAGER.get() {
        Some(manager) => manager.get_api_key(provider_id).await.ok().flatten(),
        None => None,
    }
}

/// Check if a provider has an API key (convenience function)
///
/// # Examples
/// ```no_run
/// use codetether_secrets::{has_api_key};
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// codetether_secrets::init_from_manager(codetether_secrets::SecretsManager::default()).unwrap();
/// assert!(!has_api_key("provider").await);
/// # });
/// ```
pub async fn has_api_key(provider_id: &str) -> bool {
    match SECRETS_MANAGER.get() {
        Some(manager) => manager.has_api_key(provider_id).await,
        None => false,
    }
}

/// Get full provider secrets (convenience function)
///
/// # Examples
/// ```no_run
/// use codetether_secrets::{get_provider_secrets};
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// codetether_secrets::init_from_manager(codetether_secrets::SecretsManager::default()).unwrap();
/// assert!(get_provider_secrets("provider").await.is_none());
/// # });
/// ```
pub async fn get_provider_secrets(provider_id: &str) -> Option<ProviderSecrets> {
    match SECRETS_MANAGER.get() {
        Some(manager) => manager
            .get_provider_secrets(provider_id)
            .await
            .ok()
            .flatten(),
        None => None,
    }
}
