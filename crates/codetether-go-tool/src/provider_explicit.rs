//! Resolve explicit provider names without implicit fallback.

use crate::Registry;
use anyhow::Result;
use codetether_provider_core::Provider;
use std::sync::Arc;
pub(crate) fn resolve<R: Registry>(
    registry: &R,
    provider_name: &str,
    model_name: &str,
) -> Result<(Arc<dyn Provider>, String)> {
    let normalized_provider = if provider_name == "zhipuai" {
        "zai"
    } else {
        provider_name
    };
    if let Some(provider) = registry.get(normalized_provider) {
        return Ok((provider, model_name.to_string()));
    }
    let available = registry.list().join(", ");
    anyhow::bail!(
        "Provider '{}' selected explicitly but is unavailable. Available providers: {}",
        normalized_provider,
        available
    );
}
