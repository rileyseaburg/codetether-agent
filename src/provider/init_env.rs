//! Bind shared environment registration to the instrumented native registry.
use super::{Provider, ProviderRegistry};
use std::sync::Arc;
impl codetether_provider_factory::Registry for ProviderRegistry {
    fn contains(&self, name: &str) -> bool {
        self.inner.contains(name)
    }
    fn register(&mut self, provider: Arc<dyn Provider>) {
        ProviderRegistry::register(self, provider);
    }
    fn len(&self) -> usize {
        self.inner.len()
    }
}
/// Register missing providers from environment variables in shared policy order.
pub fn register_env_fallbacks(registry: &mut ProviderRegistry) {
    codetether_provider_factory::environment::register_env_fallbacks::<
        super::init_dispatch::host::Native,
    >(registry);
}
