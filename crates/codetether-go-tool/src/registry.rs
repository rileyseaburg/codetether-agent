//! Read-only access to providers already configured by the application.

use codetether_provider_core::Provider;
use std::sync::Arc;
/// Existing provider registry used for explicit selection and ordered fallbacks.
///
/// # Implementors
/// Delegate both methods to the application's provider registry without reordering.
pub trait Registry: Send + Sync + 'static {
    /// Return the configured provider, if any.
    fn get(&self, name: &str) -> Option<Arc<dyn Provider>>;
    /// Return provider names in the existing registry order.
    fn list(&self) -> Vec<&str>;
}
