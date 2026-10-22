//! Load the incident provider with normal native credential initialization.
use anyhow::{Context, Result};
use codetether_agent::provider::{Provider, ProviderRegistry};
use std::sync::Arc;
pub(super) async fn load() -> Result<Arc<dyn Provider>> {
    let secrets = codetether_agent::secrets::SecretsManager::from_env().await?;
    codetether_agent::secrets::init_from_manager(secrets)?;
    ProviderRegistry::from_vault()
        .await?
        .get("openai-codex")
        .context("incident provider unavailable")
}
