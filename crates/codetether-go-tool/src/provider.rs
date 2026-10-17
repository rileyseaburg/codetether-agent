//! Select a configured provider using the existing fallback order.

use anyhow::Result;
use std::sync::Arc;

/// Resolve provider and model string from the registry.
/// Uses the same fallback strategy as the TUI autochat.
pub(crate) fn resolve_provider<R: crate::Registry>(
    registry: &R,
    model: &str,
) -> Result<(Arc<dyn codetether_provider_core::Provider>, String)> {
    let (provider_name, model_name) = codetether_provider_core::routing::parse_model_string(model);

    // Try explicit provider/model pair
    if let Some(provider_name) = provider_name {
        return crate::provider_explicit::resolve(registry, provider_name, model_name);
    }

    // Fallback provider order (same as TUI)
    let fallbacks = [
        "zai",
        "openai",
        "github-copilot",
        "anthropic",
        "openrouter",
        "novita",
        "moonshotai",
        "google",
    ];

    for name in fallbacks {
        if let Some(provider) = registry.get(name) {
            return Ok((provider, model.to_string()));
        }
    }

    // Last resort: first available provider
    if let Some(name) = registry.list().into_iter().next()
        && let Some(provider) = registry.get(name)
    {
        return Ok((provider, model.to_string()));
    }

    anyhow::bail!("No provider available for model '{model}' and no fallback providers found")
}
