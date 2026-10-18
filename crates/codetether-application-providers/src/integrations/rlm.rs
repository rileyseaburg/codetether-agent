//! rlm provider registry integration.
use crate::{Application, Provider, ProviderRegistry};
use std::sync::Arc;
impl<H: Application> codetether_rlm::model_resolution::Registry for ProviderRegistry<H> {
    fn resolve_model(&self, selector: &str) -> anyhow::Result<(Arc<dyn Provider>, String)> {
        ProviderRegistry::resolve_model(self, selector)
    }
}
