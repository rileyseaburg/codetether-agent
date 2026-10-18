//! Expose the existing provider registry to shared embedding selection.
use crate::provider::{Provider, ProviderRegistry};
use std::sync::Arc;
impl codetether_embedding_backends::auto::Registry for ProviderRegistry {
    fn get(&self, name: &str) -> Option<Arc<dyn Provider>> {
        ProviderRegistry::get(self, name)
    }
}
