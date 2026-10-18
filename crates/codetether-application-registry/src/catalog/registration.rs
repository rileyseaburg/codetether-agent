//! Register catalog kinds and aliases through the owning application registry.
use super::{Catalog, CatalogFactories};
use codetether_tool_construction::application::create;
use codetether_tool_registry::catalog::{Host, ToolKind, dispatch};
impl<F: CatalogFactories> Host for Catalog<'_, F> {
    fn register(&mut self, kind: ToolKind) {
        dispatch::register(self, kind);
    }
    fn register_alias(&mut self, alias: &str, kind: ToolKind) {
        self.registry
            .register_compat_alias(alias, create(kind, &F::default()));
    }
    fn register_with_alias(&mut self, alias: &str, kind: ToolKind) {
        self.registry
            .register_with_alias(alias, create(kind, &F::default()));
    }
}
