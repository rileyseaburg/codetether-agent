//! Stateless memory-embedding factory bound to an application registry type.
use std::marker::PhantomData;
/// Automatic embedding factory for an application provider registry.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_application_providers::Application>() {
/// let _factory = codetether_application_providers::embedding::AutoEmbedder::<
///     codetether_application_providers::ProviderRegistry<H>>::new();
/// # }
/// ```
pub struct AutoEmbedder<R>(PhantomData<fn() -> R>);
impl<R> AutoEmbedder<R> {
    /// Construct the stateless factory without initializing providers.
    /// # Arguments
    /// The registry binding is selected by `R`.
    /// # Returns
    /// A zero-sized factory for the selected registry type.
    /// # Examples
    /// ```
    /// let _factory = codetether_application_providers::embedding::AutoEmbedder::<()>::new();
    /// ```
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}
impl<R> Clone for AutoEmbedder<R> {
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<R> Default for AutoEmbedder<R> {
    fn default() -> Self {
        Self::new()
    }
}
impl<R> std::fmt::Debug for AutoEmbedder<R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ApplicationAutoEmbedder")
    }
}
