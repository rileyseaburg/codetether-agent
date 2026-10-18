//! spawn provider registry integration.
use crate::{Application, Provider, ProviderRegistry};
use std::sync::Arc;
impl<H: Application> codetether_spawn_policy::ModelRegistry for ProviderRegistry<H> {
    fn get(&self, provider: &str) -> Option<Arc<dyn Provider>> {
        ProviderRegistry::get(self, provider)
    }
}
