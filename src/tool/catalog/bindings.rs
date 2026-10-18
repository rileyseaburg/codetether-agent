//! Concrete command and provider constructors for the shared catalog.
use super::*;
use codetether_application_registry::catalog::CatalogFactories;
use codetether_tool_construction::provider::ProviderTool;
impl CatalogFactories for construct::Constructors {
    type Application = super::super::registry_lifecycle::Native;
    fn register_commands(registry: &mut ToolRegistry) {
        command_session::register(registry, None);
    }
    fn scripting_enabled() -> bool {
        cfg!(feature = "tetherscript")
    }
    fn provider(kind: ProviderTool, provider: Arc<dyn Provider>, model: String) -> Arc<dyn Tool> {
        provider::create(kind, provider, model)
    }
}
