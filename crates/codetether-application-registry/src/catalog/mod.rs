//! Ordered catalog registration over an application-owned tool registry.
//!
//! [`Catalog`] implements shared group dispatch and alias registration.
//! [`CatalogFactories`] supplies concrete constructors and feature choices.
//! Use [`Catalog::new`] with the shared catalog's `populate` or `Host::register`.
use crate::ToolRegistry;
use codetether_provider_core::Provider;
use std::sync::Arc;
mod dispatch;
mod factories;
mod registration;
pub use factories::CatalogFactories;

/// Registry and optional provider state used while constructing a tool catalog.
/// # Examples
/// ```no_run
/// # fn example<F: codetether_application_registry::catalog::CatalogFactories>(registry: &mut codetether_application_registry::ToolRegistry<F::Application>) {
/// let mut catalog = codetether_application_registry::catalog::Catalog::<F>::new(registry, None);
/// codetether_tool_registry::catalog::populate(&mut catalog, false, false);
/// # }
/// ```
pub struct Catalog<'a, F: CatalogFactories> {
    registry: &'a mut ToolRegistry<F::Application>,
    provider: Option<(Arc<dyn Provider>, String)>,
}
impl<'a, F: CatalogFactories> Catalog<'a, F> {
    /// Bind registry storage and optional provider/model state for construction.
    /// # Arguments
    /// `registry` receives tools; `provider` is consumed or cloned by existing provider policy.
    /// # Returns
    /// A catalog ready for group or full registration.
    /// # Examples
    /// See the [`Catalog`] example for default catalog population.
    pub fn new(
        registry: &'a mut ToolRegistry<F::Application>,
        provider: Option<(Arc<dyn Provider>, String)>,
    ) -> Self {
        Self { registry, provider }
    }
}
