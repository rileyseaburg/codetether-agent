//! Native tool registry services and catalog publication.
//!
//! [`Application`] supplies native provider search and runtime authorization to
//! the shared tool registry. [`Catalog`] binds the application's concrete tool
//! constructors. [`announce`] publishes those same schemas to the agent bus.
use codetether_application_registry::ToolRegistry;
use codetether_native_providers::ProviderRegistry;
use codetether_provider_core::Provider;
use std::{marker::PhantomData, sync::Arc};
mod application;
mod publication;
pub use publication::announce;

/// Concrete tool construction required by the native registry.
///
/// # Implementors
/// Applications populate their ordered catalog, preserving provider selection
/// and optional tool features. Search registration and policy live here.
pub trait Catalog: Sized + Send + Sync + 'static {
    /// Populate the ordered default or provider-backed tool catalog.
    ///
    /// # Arguments
    /// * `registry` - Destination native registry.
    /// * `provider` - Optional provider and model for provider-backed tools.
    ///
    /// # Returns
    /// Completes after registering the catalog's tools in their original order.
    fn populate(
        registry: &mut ToolRegistry<Application<Self>>,
        provider: Option<(Arc<dyn Provider>, String)>,
    );
}

/// Native provider, search and policy services for catalog `C`.
///
/// This marker specializes the shared registry; it is selected as a type.
///
/// # Examples
/// ```no_run
/// # fn example<C: codetether_native_registry::Catalog>() {
/// type Registry<C> = codetether_application_registry::ToolRegistry<codetether_native_registry::Application<C>>;
/// let registry = Registry::<C>::new();
/// let _definitions = registry.definitions();
/// # }
/// ```
pub struct Application<C>(PhantomData<fn() -> C>);
