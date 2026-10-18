//! Provider-first scalar registration and application command-group bindings.
use super::{Catalog, CatalogFactories};
use codetether_tool_construction::{application::create, provider};
use codetether_tool_registry::catalog::{ToolKind, dispatch::DispatchHost};
impl<F: CatalogFactories> DispatchHost for Catalog<'_, F> {
    fn register_commands(&mut self) {
        F::register_commands(self.registry);
    }
    fn scripting_enabled(&self) -> bool {
        F::scripting_enabled()
    }
    fn register_provider(&mut self, kind: ToolKind) -> bool {
        let Some(tool) = provider::create(kind, &mut self.provider, F::provider) else {
            return false;
        };
        self.registry.register(tool);
        true
    }
    fn register_scalar(&mut self, kind: ToolKind) {
        let tool = create(kind, &F::default());
        self.registry.register(tool);
    }
}
