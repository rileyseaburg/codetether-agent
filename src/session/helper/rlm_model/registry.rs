//! Native registry model lookup for shared RLM resolution.
use crate::provider::{Provider, ProviderRegistry};
use std::sync::Arc;
impl codetether_rlm::model_resolution::Registry for ProviderRegistry {
    fn resolve_model(&self, selector: &str) -> anyhow::Result<(Arc<dyn Provider>, String)> {
        ProviderRegistry::resolve_model(self, selector)
    }
}
