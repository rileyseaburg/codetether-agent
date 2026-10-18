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
        codetether_tool_construction::provider::bound::create::<
            crate::session::Session,
            codetether_rlm_tool::application::ApplicationHost<crate::session::SessionBus>,
            session_recall::host::ApplicationHost,
            ralph::host::ApplicationHost,
        >(kind, provider, model)
    }
}
