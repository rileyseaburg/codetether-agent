//! Native constructor bindings for the shared ordered tool catalog.
use super::*;
use codetether_tool_registry::catalog::{Host, ToolKind};
mod context;
mod execution;
mod files;
mod media;
mod provider;
pub(super) struct Native<'a> {
    pub registry: &'a mut ToolRegistry,
    pub provider: Option<(Arc<dyn Provider>, String)>,
}
impl Host for Native<'_> {
    fn register(&mut self, kind: ToolKind) {
        match kind {
            ToolKind::SearchGroup => register_search_tools::register(self.registry),
            ToolKind::CommandGroup => command_session::register(self.registry, None),
            ToolKind::GoalGroup => goal::register(self.registry),
            ToolKind::MediaGroup => register_media::register(self.registry),
            ToolKind::CollaborationGroup => collaboration::register(self.registry),
            ToolKind::ScriptingGroup => {
                #[cfg(feature = "tetherscript")]
                tetherscript::register(self.registry);
            }
            ToolKind::CompatibilityGroup => self.registry.register_compat_aliases(),
            _ => {
                if provider::register(self, kind) {
                    return;
                }
                let tool = files::create(kind)
                    .or_else(|| context::create(kind))
                    .or_else(|| execution::create(kind))
                    .or_else(|| media::create(kind))
                    .expect("native catalog constructor");
                self.registry.register(tool);
            }
        }
    }
}
