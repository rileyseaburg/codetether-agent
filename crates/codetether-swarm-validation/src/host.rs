//! Application registry loading for provider validation.
use anyhow::Result;
use codetether_model_discovery::Registry;
use std::future::Future;

/// Supply configured providers to the preflight validator.
///
/// # Implementors
/// Applications retain their existing credential loading and provider setup.
pub trait Host {
    /// Registry supporting provider enumeration and lookup.
    type Registry: Registry;
    /// Load the provider registry, returning initialization failures unchanged.
    fn registry() -> impl Future<Output = Result<Self::Registry>>;
}
