//! Resolve the configured or default model and retain its provider registry.
use super::*;
use std::sync::Arc;
pub(super) async fn prepare(
    args: &ForageArgs,
) -> Result<(ProviderRegistry, Arc<dyn crate::provider::Provider>, String)> {
    let model = args
        .model
        .clone()
        .unwrap_or_else(|| "anthropic/claude-sonnet-4-20250514".to_string());

    // Load provider registry and get a provider
    let registry = ProviderRegistry::from_vault()
        .await
        .context("Failed to load provider registry for go engine")?;

    let (provider, resolved_model) = registry
        .resolve_model(&model)
        .with_context(|| format!("Failed to resolve model '{}' for go engine", model))?;

    Ok((registry, provider, resolved_model))
}
