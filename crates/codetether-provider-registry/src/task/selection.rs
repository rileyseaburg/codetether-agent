//! Provider-registry loading for worker sessions.

use anyhow::{Context, Result};

use super::Host;
use crate::{PerTaskProviderKeys, ProviderRegistry};

/// Select usable task credentials before falling back to the platform cache.
///
/// # Arguments
/// * `provider_keys` — Optional provider-key claim from the task's session.
///
/// # Returns
/// An owned task registry or a clone of the cached platform registry.
///
/// # Errors
/// Returns platform-cache loading failures with the existing Vault context.
/// Malformed task credentials retain the existing fallback behavior.
///
/// # Examples
/// ```no_run
/// # async fn load<H: codetether_provider_registry::task::Host>() -> anyhow::Result<()> {
/// let _registry = codetether_provider_registry::task::load_task_provider_registry::<H>(None).await?;
/// # Ok(())
/// # }
/// ```
pub async fn load_task_provider_registry<H: Host>(
    provider_keys: Option<&serde_json::Value>,
) -> Result<ProviderRegistry> {
    let per_task_keys = provider_keys
        .and_then(|value| PerTaskProviderKeys::from_value(value).ok())
        .flatten();
    if let Some(keys) = per_task_keys {
        let registry = keys.build_registry::<H>();
        if !registry.list().is_empty() {
            tracing::info!(source = keys.source(), providers = ?keys.provider_names(), "Using tenant-scoped per-task provider registry");
            return Ok(registry);
        }
        tracing::warn!(
            "Per-task provider key payload produced no providers; falling back to platform registry"
        );
    }
    Ok(H::platform_registry()
        .await
        .context("Failed to load provider registry from Vault — check VAULT_ADDR/VAULT_TOKEN")?)
}
