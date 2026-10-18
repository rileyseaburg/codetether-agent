//! Application registry initialization for worker model discovery.
use crate::Registry;
use anyhow::Result;
use std::{future::Future, sync::Arc};

/// Load primary and fallback provider registries without exposing credentials.
///
/// # Implementors
/// Applications retain their shared credential loading and fallback config.
pub trait Host {
    /// Provider lookup implementation; clones preserve provider handles.
    type Registry: Registry + Clone;
    /// Load the shared primary registry.
    fn shared() -> impl Future<Output = Result<Arc<Self::Registry>>>;
    /// Build a fallback registry if the primary is unavailable or empty.
    fn fallback() -> impl Future<Output = Result<Self::Registry>>;
}
