//! Provider discovery boundary for retrying PRD generation.

use codetether_provider_core::Provider;
use std::sync::Arc;
/// Read-only provider lookup for generation retries.
///
/// # Implementors
/// Delegate to an existing configured registry without creating credentials.
pub trait Registry: Send + Sync {
    /// Return the configured provider for a known name.
    fn get(&self, name: &str) -> Option<Arc<dyn Provider>>;
}
