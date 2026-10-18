//! Native subcall cache inputs retained by reset-policy derivation.
use codetether_provider_core::Provider;
use std::sync::Arc;
/// Extends read-only legacy input with the native subcall cache.
/// # Implementors
/// Return the same runtime-only fields used by native session derivation.
pub trait Source: codetether_context_legacy::Source {
    /// Cached provider for subcalls, if one has already been resolved.
    fn subcall_provider(&self) -> Option<&Arc<dyn Provider>>;
    /// Cached provider-local subcall model name.
    fn subcall_model_name(&self) -> Option<&str>;
}
