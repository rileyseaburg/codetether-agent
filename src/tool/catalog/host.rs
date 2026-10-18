//! Bind shared catalog groups to the native tool registry.
use super::*;
pub(in crate::tool) struct Native<'a> {
    pub registry: &'a mut ToolRegistry,
    pub provider: Option<(Arc<dyn Provider>, String)>,
}
impl Host for Native<'_> {
    fn register(&mut self, kind: ToolKind) {
        codetether_tool_registry::catalog::dispatch::register(self, kind);
    }
    fn register_alias(&mut self, alias: &str, kind: ToolKind) {
        self.registry
            .register_compat_alias(alias, construct::create(kind));
    }
    fn register_with_alias(&mut self, alias: &str, kind: ToolKind) {
        self.registry
            .register_with_alias(alias, construct::create(kind));
    }
}

impl codetether_tool_registry::catalog::dispatch::DispatchHost for Native<'_> {
    fn register_commands(&mut self) {
        command_session::register(self.registry, None);
    }
    fn scripting_enabled(&self) -> bool {
        cfg!(feature = "tetherscript")
    }
    fn register_provider(&mut self, kind: ToolKind) -> bool {
        provider::register(self, kind)
    }
    fn register_scalar(&mut self, kind: ToolKind) {
        let tool = construct::create(kind);
        self.registry.register(tool);
    }
}
