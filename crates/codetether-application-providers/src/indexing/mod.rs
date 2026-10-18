//! Indexer embedding resolution through the application's provider factory.
//!
//! [Resolver] loads the same configured provider registry used elsewhere in
//! the application. Supply it to `codetether_indexer::build_index_with_resolver`
//! or the indexer CLI; provider adaptation preserves the original embed call.
use std::marker::PhantomData;
mod provider;
mod resolver;
/// Indexer provider resolver bound to an application registry type.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_application_providers::Application>() {
/// let _resolver = codetether_application_providers::indexing::Resolver::<codetether_application_providers::ProviderRegistry<H>>::new();
/// # }
/// ```
pub struct Resolver<R>(PhantomData<fn() -> R>);
impl<R> Resolver<R> {
    /// Construct a stateless resolver without loading providers.
    /// # Arguments
    /// The provider registry type is selected by `R`.
    /// # Returns
    /// A resolver ready for indexer requests.
    /// # Examples
    /// ```
    /// let _resolver = codetether_application_providers::indexing::Resolver::<()>::new();
    /// ```
    pub const fn new() -> Self {
        Self(PhantomData)
    }
}
impl<R> Default for Resolver<R> {
    fn default() -> Self {
        Self::new()
    }
}
