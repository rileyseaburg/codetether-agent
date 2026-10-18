//! go provider registry integration.
use crate::{Application, Provider, ProviderRegistry};
use std::sync::Arc;
impl<H: Application> codetether_go_tool::Registry for ProviderRegistry<H> {
    fn get(&self, name: &str) -> Option<Arc<dyn Provider>> {
        ProviderRegistry::get(self, name)
    }
    fn list(&self) -> Vec<&str> {
        ProviderRegistry::list(self)
    }
}
