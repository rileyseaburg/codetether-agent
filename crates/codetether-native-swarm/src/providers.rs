//! Application provider-registry operations used by the shared orchestrator.
use codetether_native_providers::ProviderRegistry;
use codetether_provider_core::Provider;
use std::sync::Arc;
/// Native provider registry binding for swarm orchestration.
pub struct Providers;
impl codetether_swarm_orchestrator::Host for Providers {
    type Providers = ProviderRegistry;
    async fn load() -> anyhow::Result<ProviderRegistry> {
        ProviderRegistry::from_vault().await
    }
    fn list(providers: &ProviderRegistry) -> Vec<&str> {
        providers.list()
    }
    fn get(providers: &ProviderRegistry, name: &str) -> Option<Arc<dyn Provider>> {
        providers.get(name)
    }
}

impl codetether_swarm_validation::Host for Providers {
    type Registry = ProviderRegistry;
    async fn registry() -> anyhow::Result<ProviderRegistry> {
        ProviderRegistry::from_vault().await
    }
}
