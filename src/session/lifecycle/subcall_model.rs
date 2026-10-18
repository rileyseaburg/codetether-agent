//! Native registry bindings for shared subcall model policy.
use crate::provider::{Provider, ProviderRegistry};
use crate::session::Session;
use anyhow::Result;
use std::sync::Arc;
impl codetether_session_model_policy::ModelResolver for ProviderRegistry {
    fn resolve_model(&self, selector: &str) -> Result<(Arc<dyn Provider>, String)> {
        ProviderRegistry::resolve_model(self, selector)
    }
}
impl Session {
    /// Resolve the configured subcall model unless a provider is already cached.
    pub fn resolve_subcall_provider(&mut self, registry: &ProviderRegistry) {
        codetether_session_model_policy::resolve_subcall_provider(&mut self.metadata, registry);
    }
}
