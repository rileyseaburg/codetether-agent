//! Bind shared catalog groups to the native tool registry.
use super::*;
pub(in crate::tool) struct Native<'a> {
    pub registry: &'a mut ToolRegistry,
    pub provider: Option<(Arc<dyn Provider>, String)>,
}
impl Host for Native<'_> {
    fn register(&mut self, kind: ToolKind) {
        match kind {
            ToolKind::SearchGroup => codetether_tool_registry::catalog::groups::search(self),
            ToolKind::CommandGroup => command_session::register(self.registry, None),
            ToolKind::GoalGroup => codetether_tool_registry::catalog::groups::goals(self),
            ToolKind::MediaGroup => codetether_tool_registry::catalog::groups::media(self),
            ToolKind::CollaborationGroup => {
                codetether_tool_registry::catalog::groups::collaboration(self)
            }
            ToolKind::ScriptingGroup => {
                #[cfg(feature = "tetherscript")]
                codetether_tool_registry::catalog::groups::scripting(self);
            }
            ToolKind::CompatibilityGroup => {
                codetether_tool_registry::catalog::groups::compatibility(self)
            }
            _ => {
                if provider::register(self, kind) {
                    return;
                }
                let tool = construct::create(kind);
                self.registry.register(tool);
            }
        }
    }
    fn register_alias(&mut self, alias: &str, kind: ToolKind) {
        self.registry
            .inner
            .register_compat_alias(alias, construct::create(kind));
    }
    fn register_with_alias(&mut self, alias: &str, kind: ToolKind) {
        self.registry
            .inner
            .register_with_alias(alias, construct::create(kind));
    }
}
