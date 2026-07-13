//! Capability collection from configured providers.

use crate::provider::ProviderRegistry;

use super::types::ProviderCapability;

pub(super) async fn capabilities(
    registry: &ProviderRegistry,
    filter: Option<&str>,
) -> Vec<ProviderCapability> {
    let mut capabilities = Vec::new();
    for name in registry.list() {
        if filter.is_some_and(|selected| selected != name) {
            continue;
        }
        capabilities.push(collect_provider(registry, name).await);
    }
    capabilities.sort_by(|left, right| left.provider.cmp(&right.provider));
    capabilities
}

async fn collect_provider(registry: &ProviderRegistry, name: &str) -> ProviderCapability {
    let result = match registry.get(name) {
        Some(provider) => provider.list_models().await,
        None => Err(anyhow::anyhow!("provider disappeared during discovery")),
    };
    match result {
        Ok(mut models) => {
            models.sort_by(|left, right| left.id.cmp(&right.id));
            ProviderCapability {
                provider: name.to_string(),
                available: true,
                source: "configured provider registry",
                error: None,
                models: models.into_iter().map(super::enrich::capability).collect(),
            }
        }
        Err(error) => ProviderCapability {
            provider: name.to_string(),
            available: false,
            source: "configured provider registry",
            error: Some(error.to_string()),
            models: Vec::new(),
        },
    }
}
