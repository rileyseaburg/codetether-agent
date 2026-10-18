//! forage provider registry integration.
use crate::{Application, Provider, ProviderRegistry};
use std::sync::Arc;
impl<H: Application> codetether_forage_execution::Providers for ProviderRegistry<H> {
    fn resolve_model(&self, model: &str) -> anyhow::Result<(Arc<dyn Provider>, String)> {
        ProviderRegistry::resolve_model(self, model)
    }
}
