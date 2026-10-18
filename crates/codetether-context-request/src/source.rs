//! Synchronous preparation and policy inputs supplied by the native session.
use codetether_context_policy::derivation::DerivePolicy;
use codetether_provider_core::Provider;
use std::sync::Arc;
/// Session identity and synchronous request preparation.
/// # Implementors
/// Delegate to the existing session policy and proactive preparation logic.
pub trait Source: Send + Sync {
    /// Owning session identifier for provider scoping.
    fn session_id(&self) -> &str;
    /// Resolve the current effective derivation policy.
    fn effective_policy(&self, model: &str) -> DerivePolicy;
    /// Start proactive context preparation before deriving a request.
    fn prepare(&self, provider: Arc<dyn Provider>, model: &str);
}
