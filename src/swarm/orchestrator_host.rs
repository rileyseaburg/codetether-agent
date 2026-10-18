//! Application provider-registry operations used by the shared orchestrator.
use crate::provider::{Provider, ProviderRegistry};
use std::sync::Arc;
/// Native provider registry binding for swarm orchestration.
pub struct Native;
impl codetether_swarm_orchestrator::Host for Native {
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
