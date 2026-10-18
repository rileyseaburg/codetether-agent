//! Native registry bindings for shared subcall model policy.
use crate::provider::ProviderRegistry;
use crate::session::Session;
impl Session {
    /// Resolve the configured subcall model unless a provider is already cached.
    pub fn resolve_subcall_provider(&mut self, registry: &ProviderRegistry) {
        codetether_session_model_policy::resolve_subcall_provider(&mut self.metadata, registry);
    }
}
