//! Resolve configured RLM model choices without owning provider credentials.
//!
//! [resolve] and [resolve_with_score] retain the caller provider when no model
//! change is needed or registry resolution fails. Hosts supply a deferred loader
//! for their [Registry]; the loader is called only for a different model choice.
mod resolve;
mod scored;
use codetether_provider_core::Provider;
pub use resolve::resolve;
pub use scored::resolve_with_score;
use std::sync::Arc;
/// Model lookup required after RLM policy selects a different model.
/// # Implementors
/// Application registries expose their existing selector resolution operation.
pub trait Registry: Send + Sync {
    /// Resolve a selector to the existing provider and normalized model name.
    /// # Errors
    /// Returns the registry's original lookup failure.
    fn resolve_model(&self, selector: &str) -> anyhow::Result<(Arc<dyn Provider>, String)>;
}
