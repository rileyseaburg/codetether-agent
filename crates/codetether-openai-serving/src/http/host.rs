//! Provider lookup contracts for OpenAI HTTP request handling.
use codetether_provider_core::Provider;
use std::{fmt::Display, future::Future, sync::Arc};

/// Registry operations needed to select an OpenAI request's provider.
/// # Implementors
/// The application registry supplies its existing provider names and instances.
pub trait Registry: Send + Sync + 'static {
    /// List provider names available for request model resolution.
    fn list(&self) -> Vec<&str>;
    /// Retrieve a provider by its resolved name.
    fn get(&self, name: &str) -> Option<Arc<dyn Provider>>;
}

/// Application binding for loading the shared provider registry.
/// # Implementors
/// Implement this on a marker type using the application's credential loader.
pub trait Host: Send + Sync + 'static {
    /// Registry used for provider selection.
    type Registry: Registry;
    /// Loading failure displayed in the existing OpenAI error envelope.
    type Error: Display;
    /// Load or reuse the application's shared provider registry.
    fn load_registry() -> impl Future<Output = Result<Arc<Self::Registry>, Self::Error>> + Send;
}
