//! Bind interactive Ralph selection to the native provider registry.
use crate::provider::{Provider, ProviderRegistry};
use std::sync::Arc;
impl codetether_ralph_cli::interactive::Registry for ProviderRegistry {
    fn resolve_model(&self, model: &str) -> anyhow::Result<(Arc<dyn Provider>, String)> {
        ProviderRegistry::resolve_model(self, model)
    }
}
