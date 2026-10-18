//! factory provider registry integration.
use crate::{Application, Provider, ProviderRegistry};
use std::sync::Arc;
impl<H: Application> codetether_provider_factory::Registry for ProviderRegistry<H> {
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
