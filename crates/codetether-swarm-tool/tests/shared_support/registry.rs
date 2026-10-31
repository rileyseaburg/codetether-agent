//! Actual workspace tool installation into a minimal test catalog.
use super::noop::NoLsp;
use codetether_tool_core::Tool;
use std::{collections::HashMap, sync::Arc};
#[derive(Default)]
pub struct Catalog(pub HashMap<String, Arc<dyn Tool>>);
impl codetether_workspace_tools::Registry for Catalog {
    fn register(&mut self, tool: Arc<dyn Tool>) {
        self.0.insert(tool.id().into(), tool);
    }
    fn unregister(&mut self, id: &str) {
        self.0.remove(id);
    }
    fn list(&self) -> Vec<&str> {
        self.0.keys().map(String::as_str).collect()
    }
}
impl codetether_workspace_tools::Host for super::TestHost {
    type Registry = Catalog;
    fn lsp(_: String) -> Arc<dyn Tool> {
        Arc::new(NoLsp)
    }
    fn scripting(_: std::path::PathBuf) -> Option<Arc<dyn Tool>> {
        None
    }
}
pub(super) fn definitions(registry: &Catalog) -> Vec<codetether_provider_core::ToolDefinition> {
    registry
        .0
        .values()
        .map(|tool| codetether_provider_core::ToolDefinition {
            name: tool.id().into(),
            description: tool.description().into(),
            parameters: tool.parameters(),
        })
        .collect()
}
