//! discovery provider registry integration.
use crate::{Application, Provider, ProviderRegistry};
use std::sync::Arc;
impl<H: Application> codetether_model_discovery::Registry for ProviderRegistry<H> {
    fn list(&self) -> Vec<&str> {
        ProviderRegistry::list(self)
    }
    fn get(&self, name: &str) -> Option<Arc<dyn Provider>> {
        ProviderRegistry::get(self, name)
    }
}
