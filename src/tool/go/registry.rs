//! Read-only binding to the application's configured provider registry.
use crate::provider::{Provider, ProviderRegistry};
use std::sync::Arc;
impl codetether_go_tool::Registry for ProviderRegistry {
    fn get(&self, name: &str) -> Option<Arc<dyn Provider>> {
        ProviderRegistry::get(self, name)
    }
    fn list(&self) -> Vec<&str> {
        ProviderRegistry::list(self)
    }
}
