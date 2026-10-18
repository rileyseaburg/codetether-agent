//! ralph provider registry integration.
use crate::{Application, Provider, ProviderRegistry};
use std::sync::Arc;
impl<H: Application> codetether_ralph_cli::interactive::Registry for ProviderRegistry<H> {
    fn resolve_model(&self, model: &str) -> anyhow::Result<(Arc<dyn Provider>, String)> {
        ProviderRegistry::resolve_model(self, model)
    }
}
