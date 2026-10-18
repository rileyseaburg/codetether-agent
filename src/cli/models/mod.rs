//! Native provider initialization for shared model discovery.
use super::ModelsArgs;
use crate::provider::ProviderRegistry;

pub async fn execute(args: ModelsArgs) -> anyhow::Result<()> {
    let registry = ProviderRegistry::from_vault().await?;
    codetether_models_cli::execute(args, &Registry(registry)).await
}

struct Registry(ProviderRegistry);
impl codetether_models_cli::Registry for Registry {
    fn list(&self) -> Vec<&str> {
        self.0.list()
    }
    fn get(&self, name: &str) -> Option<std::sync::Arc<dyn codetether_provider_core::Provider>> {
        self.0.get(name)
    }
}
