//! Native provider credentials for shared swarm preflight validation.
use crate::provider::ProviderRegistry;
use anyhow::Result;

pub struct Native;
impl codetether_swarm_validation::Host for Native {
    type Registry = ProviderRegistry;
    async fn registry() -> Result<ProviderRegistry> {
        ProviderRegistry::from_vault().await
    }
}
