//! Thinker service bindings using application providers and Bedrock credentials.
//!
//! [`ApplicationHost`] supplies the thinker service's provider operations.
//! Registry completion uses the application's shared Vault-backed cache, while
//! the Bedrock branch retains its environment credential and region selection.
use std::marker::PhantomData;
mod bedrock;
mod host;
mod registry;

/// Provider operations for a thinker using application registry policy `H`.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_application_providers::thinker::Registry>(config: codetether_cognition_core::ThinkerConfig) -> anyhow::Result<()> {
/// let _client = codetether_cognition_core::thinker::client::Client::<
///     codetether_thinker_service::Service<codetether_application_providers::thinker::ApplicationHost<H>>
/// >::new(config)?;
/// # Ok(()) }
/// ```
pub struct ApplicationHost<H>(PhantomData<fn() -> H>);

/// Registry completion supplied independently of native model construction.
/// # Implementors
/// Applications bind their cached provider selection here without requiring
/// the thinker service to resolve the provider factory's construction bounds.
pub trait Registry {
    /// Complete one thought using the selected application provider.
    /// # Errors
    /// Propagates provider initialization, model lookup and completion errors.
    fn think(
        config: &codetether_cognition_core::ThinkerConfig,
        system: &str,
        user: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<codetether_cognition_core::ThinkerOutput>> + Send;
}

pub use registry::think;
