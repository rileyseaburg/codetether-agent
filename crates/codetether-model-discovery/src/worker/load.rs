//! Sequential worker inventory discovery and registry fallback selection.
use super::{Host, Models};
use crate::Registry;
use anyhow::Result;

pub(super) async fn uncached<H: Host>() -> Result<Models> {
    let registry = match H::shared().await {
        Ok(registry) if !registry.list().is_empty() => (*registry).clone(),
        Ok(_) | Err(_) => H::fallback().await?,
    };
    let mut models_by_provider = Models::new();
    for provider_name in registry.list() {
        if let Some(provider) = registry.get(provider_name)
            && let Ok(models) = provider.list_models().await
            && !models.is_empty()
        {
            models_by_provider.insert(provider_name.to_string(), models);
        }
    }
    Ok(models_by_provider)
}
