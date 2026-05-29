//! Provider-registry loading for worker sessions.

use anyhow::{Context, Result};

use crate::provider::ProviderRegistry;
use crate::session::Session;

pub(super) async fn load_task_provider_registry(session: &Session) -> Result<ProviderRegistry> {
    let per_task_keys = session
        .metadata
        .provider_keys
        .as_ref()
        .and_then(|value| crate::provider::PerTaskProviderKeys::from_value(value).ok())
        .flatten();
    if let Some(keys) = per_task_keys {
        let registry = keys.build_registry();
        if !registry.list().is_empty() {
            tracing::info!(source = keys.source(), providers = ?keys.provider_names(), "Using tenant-scoped per-task provider registry");
            return Ok(registry);
        }
        tracing::warn!(
            "Per-task provider key payload produced no providers; falling back to platform registry"
        );
    }
    Ok((*ProviderRegistry::shared_from_vault()
        .await
        .context("Failed to load provider registry from Vault — check VAULT_ADDR/VAULT_TOKEN")?)
    .clone())
}
