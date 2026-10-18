//! Application provider constructors composed from shared implementations.
//!
//! [ApplicationHost] owns Bedrock and Copilot construction while the caller
//! supplies its Copilot identity and [LocalProvider] runtime adapter. The
//! existing factory [crate::Host] policy and dispatch order are unchanged.
use codetether_provider_copilot::application::Identity;
use codetether_provider_core::Provider;
use std::{marker::PhantomData, sync::Arc};
mod implementation;
/// Runtime-dependent local provider construction supplied by the application.
/// # Implementors
/// Applications bind their existing feature-selected local CUDA provider.
pub trait LocalProvider {
    /// Construct a local provider using the selected model.
    fn create(model: String) -> anyhow::Result<Arc<dyn Provider>>;
    /// Construct a local provider with explicit model and tokenizer artifacts.
    fn create_with_paths(
        model: String,
        path: String,
        tokenizer: Option<String>,
        architecture: Option<String>,
    ) -> anyhow::Result<Arc<dyn Provider>>;
}
/// Provider factory using caller identity and a local runtime constructor.
/// # Examples
/// ```no_run
/// # fn create<I: codetether_provider_copilot::application::Identity, L: codetether_provider_factory::application::LocalProvider>(key: String) -> anyhow::Result<()> {
/// use codetether_provider_factory::{Host, application::ApplicationHost};
/// let _provider = ApplicationHost::<I, L>::copilot_new(key)?;
/// # Ok(()) }
/// ```
pub struct ApplicationHost<I: Identity, L: LocalProvider>(PhantomData<fn() -> (I, L)>);
