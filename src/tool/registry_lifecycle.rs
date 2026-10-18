//! Application catalog construction for the native registry services.
use super::{ToolRegistry, catalog};
use crate::provider::Provider;
use std::sync::Arc;

/// Native registry specialization using the application's tool constructors.
pub type Native = codetether_native_registry::Application<Catalog>;
/// Application-specific catalog population required by native registry services.
pub struct Catalog;
impl codetether_native_registry::Catalog for Catalog {
    fn populate(registry: &mut ToolRegistry, provider: Option<(Arc<dyn Provider>, String)>) {
        let provider_backed = provider.is_some();
        codetether_tool_registry::catalog::populate(
            &mut catalog::Native::new(registry, provider),
            provider_backed,
            cfg!(feature = "tetherscript"),
        );
    }
}
