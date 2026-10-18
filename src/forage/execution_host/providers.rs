//! Preserve application provider initialization and resolution.
use crate::provider::{Provider, ProviderRegistry};
use std::sync::Arc;
impl codetether_forage_execution::Providers for ProviderRegistry {
    fn resolve_model(&self, model: &str) -> anyhow::Result<(Arc<dyn Provider>, String)> {
        ProviderRegistry::resolve_model(self, model)
    }
}
