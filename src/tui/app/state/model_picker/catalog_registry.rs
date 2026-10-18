//! Supply configured application providers to shared catalog discovery.
use crate::provider::{Provider, ProviderRegistry};
use std::sync::Arc;
impl codetether_model_discovery::Registry for ProviderRegistry {
    fn list(&self) -> Vec<&str> {
        ProviderRegistry::list(self)
    }
    fn get(&self, name: &str) -> Option<Arc<dyn Provider>> {
        ProviderRegistry::get(self, name)
    }
}
