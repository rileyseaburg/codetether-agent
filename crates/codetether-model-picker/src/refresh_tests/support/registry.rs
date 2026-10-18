//! Instrumented shared registry exposed through the discovery interface.
use codetether_provider_core::Provider;
use std::sync::Arc;
pub(in crate::refresh_tests) struct Registry(codetether_provider_registry::ProviderRegistry);
impl Registry {
    pub(in crate::refresh_tests) fn new() -> Self {
        Self(codetether_provider_registry::ProviderRegistry::new())
    }
    pub(in crate::refresh_tests) fn register(&mut self, provider: Arc<dyn Provider>) {
        self.0.register(provider);
    }
}
impl codetether_model_discovery::Registry for Registry {
    fn list(&self) -> Vec<&str> {
        self.0.list()
    }
    fn get(&self, name: &str) -> Option<Arc<dyn Provider>> {
        self.0.get(name)
    }
}
