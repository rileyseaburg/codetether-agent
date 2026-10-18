//! Worker provider lookup bound to the application's configured registry.
use super::{Provider, ProviderRegistry};
use std::sync::Arc;

impl codetether_a2a_worker_core::execution_policy::Registry for ProviderRegistry {
    fn list(&self) -> Vec<&str> {
        ProviderRegistry::list(self)
    }
    fn get(&self, name: &str) -> Option<Arc<dyn Provider>> {
        ProviderRegistry::get(self, name)
    }
}
