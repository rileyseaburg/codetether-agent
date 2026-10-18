//! Backend construction and execution required by the local CUDA provider.
use codetether_cognition_core::{ThinkerConfig, ThinkerOutput};
use std::future::Future;
/// Application-supplied backend used after provider configuration is resolved.
pub trait Runtime: Send + Sync + Sized + 'static {
    /// Construct the backend for the resolved model configuration.
    /// # Errors
    /// Returns backend initialization errors without adding prompt text.
    fn new(config: ThinkerConfig) -> anyhow::Result<Self>;
    /// Execute a provider completion with the configured backend.
    /// # Errors
    /// Returns the backend inference failure.
    fn think_as_provider(
        &self,
        provider: &str,
        system: &str,
        user: &str,
    ) -> impl Future<Output = anyhow::Result<ThinkerOutput>> + Send;
}
