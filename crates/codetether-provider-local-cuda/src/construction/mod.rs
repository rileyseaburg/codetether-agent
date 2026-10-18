//! Boxed local-provider construction for application provider factories.
//!
//! [`LocalProvider`] preserves a common constructor interface across CUDA and
//! disabled builds. The selected [`crate::LocalCudaProvider`] implements it;
//! callers receive the same provider or initialization error as its constructors.
use codetether_provider_core::Provider;
use std::sync::Arc;
#[cfg(not(feature = "candle-cuda"))]
mod disabled;
#[cfg(feature = "candle-cuda")]
mod enabled;

/// Runtime-dependent local provider construction supplied to a provider factory.
/// # Implementors
/// [`crate::LocalCudaProvider`] uses the backend selected by this crate's features.
pub trait LocalProvider {
    /// Construct a local provider using the selected model.
    /// # Errors
    /// Returns the provider's initialization error, including unavailable CUDA.
    fn create(model: String) -> anyhow::Result<Arc<dyn Provider>>;
    /// Construct a local provider with explicit model and tokenizer artifacts.
    /// # Errors
    /// Returns the provider's artifact or initialization error.
    fn create_with_paths(
        model: String,
        path: String,
        tokenizer: Option<String>,
        architecture: Option<String>,
    ) -> anyhow::Result<Arc<dyn Provider>>;
}
