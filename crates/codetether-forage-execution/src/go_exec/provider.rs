//! Resolve the configured or default model and retain its provider registry.
use crate::*;
use anyhow::Context;
use std::sync::Arc;
pub(super) async fn prepare<H: Host>(
    args: &ForageArgs,
) -> Result<(
    H::Registry,
    Arc<dyn codetether_provider_core::Provider>,
    String,
)> {
    let model = args
        .model
        .clone()
        .unwrap_or_else(|| "anthropic/claude-sonnet-4-20250514".to_string());

    // Load provider registry and get a provider
    let registry = H::providers()
        .await
        .context("Failed to load provider registry for go engine")?;

    let (provider, resolved_model) = registry
        .resolve_model(&model)
        .with_context(|| format!("Failed to resolve model '{}' for go engine", model))?;

    Ok((registry, provider, resolved_model))
}
