//! Bridge shared factory population to instrumented registry storage.
use super::*;
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
impl ProviderRegistry {
    /// Whether the exact provider key is already registered.
    pub fn contains(&self, name: &str) -> bool {
        self.providers.contains_key(name)
    }
    /// Number of registered provider keys.
    pub fn len(&self) -> usize {
        self.providers.len()
    }
    /// Whether the collection has no providers.
    pub fn is_empty(&self) -> bool {
        self.providers.is_empty()
    }
}
