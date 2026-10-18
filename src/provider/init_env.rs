//! Bind shared environment registration to the instrumented native registry.
use super::{Provider, ProviderRegistry};
use std::sync::Arc;
impl codetether_provider_factory::Registry for ProviderRegistry {
    fn contains(&self, name: &str) -> bool {
        self.providers.contains_key(name)
    }
    fn register(&mut self, provider: Arc<dyn Provider>) {
        ProviderRegistry::register(self, provider);
    }
    fn len(&self) -> usize {
        self.providers.len()
    }
}
/// Register missing providers from environment variables in shared policy order.
pub fn register_env_fallbacks(registry: &mut ProviderRegistry) {
    codetether_provider_factory::environment::register_env_fallbacks::<
        super::init_dispatch::host::Native,
    >(registry);
}
