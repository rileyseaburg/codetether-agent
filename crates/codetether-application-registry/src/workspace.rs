//! Shared workspace registry integration.
use super::{Application, ToolRegistry};
use codetether_tool_core::Tool;
use std::sync::Arc;
impl<H: Application> codetether_workspace_tools::Registry for ToolRegistry<H> {
    fn register(&mut self, tool: Arc<dyn Tool>) {
        ToolRegistry::register(self, tool);
    }
    fn unregister(&mut self, id: &str) {
        ToolRegistry::unregister(self, id);
    }
    fn list(&self) -> Vec<&str> {
        ToolRegistry::list(self)
    }
}
