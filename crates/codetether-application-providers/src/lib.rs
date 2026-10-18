//! Application-bound provider collections, initialization and consumer bindings.
//!
//! [ProviderRegistry] owns the shared instrumented provider registry.
//! [Application] selects the native factory and supplies the process-wide cache;
//! the registry owns configuration/Vault/task construction and consumer traits.
use anyhow::Result;
use codetether_provider_core::Provider;
use codetether_secrets::ProviderSecrets;
use std::{collections::HashMap, marker::PhantomData, sync::Arc};
mod application;
pub use application::Application;
mod collection;
mod config;
mod convert;
mod create;
mod integrations;
mod task;
pub mod task_factory;
mod vault;

/// Provider registry bound to the embedding application's factory and cache.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_application_providers::Application>() {
/// let registry = codetether_application_providers::ProviderRegistry::<H>::new();
/// assert!(registry.list().is_empty());
/// # }
/// ```
pub struct ProviderRegistry<H: Application> {
    inner: codetether_provider_registry::ProviderRegistry,
    application: PhantomData<fn() -> H>,
}
impl<H: Application> Clone for ProviderRegistry<H> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            application: PhantomData,
        }
    }
}
impl<H: Application> std::fmt::Debug for ProviderRegistry<H> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Debug::fmt(&self.inner, f)
    }
}

/// Application-backed local and cloud memory embedding selection.
pub mod embedding;
/// Auto-chat rotation using the application registry.
pub mod rotation;
/// Router model resolution through the application registry.
pub mod routing;

/// Search orchestration using the application provider and memory policies.
pub mod search;

/// Background RLM routing with application model policy and session progress.
pub mod background;
/// Morph editing through the application's cached provider registry.
pub mod editing;
/// Provider-backed embedding resolution for source indexes.
pub mod indexing;
/// RLM model choices resolved through the application's cached providers.
pub mod model_resolution;
/// Credentialed thinker backends and cached provider-registry completion.
pub mod thinker;
/// Worker swarm settings using the application provider cache.
pub mod worker_swarm;
