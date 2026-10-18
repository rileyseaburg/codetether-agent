//! Native constructor bindings for the shared ordered tool catalog.
use super::*;
use codetether_tool_registry::catalog::{Host, ToolKind};
mod construct;
mod context;
mod execution;
mod files;
mod integrations;
mod media;
mod provider;
mod search_tools;
pub(super) struct Native<'a> {
    pub registry: &'a mut ToolRegistry,
    pub provider: Option<(Arc<dyn Provider>, String)>,
}
impl Host for Native<'_> {
    fn register(&mut self, kind: ToolKind) {
        match kind {
            ToolKind::SearchGroup => codetether_tool_registry::catalog::groups::search(self),
            ToolKind::CommandGroup => command_session::register(self.registry, None),
            ToolKind::GoalGroup => goal::register(self.registry),
            ToolKind::MediaGroup => codetether_tool_registry::catalog::groups::media(self),
            ToolKind::CollaborationGroup => collaboration::register(self.registry),
            ToolKind::ScriptingGroup => {
                #[cfg(feature = "tetherscript")]
                tetherscript::register(self.registry);
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
