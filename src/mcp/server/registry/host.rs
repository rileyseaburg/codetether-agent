//! Keep policy wrapping on native executable tools.
use crate::tool::{Tool, ToolRegistry};
use codetether_mcp_core::server_catalog::McpToolHandler;
use serde_json::Value;
use std::sync::Arc;
pub(super) struct Registry<'a>(pub &'a ToolRegistry);
impl codetether_mcp_server::registry::Host for Registry<'_> {
    fn list(&self) -> Vec<&str> {
        self.0.list()
    }
    fn get(&self, id: &str) -> Option<Arc<dyn Tool>> {
        self.0.get(id)
    }
    fn handler(tool_name: String, tool_clone: Arc<dyn Tool>) -> McpToolHandler {
        Arc::new(move |args: Value| {
            crate::mcp::tool_policy::call_registered(
                tool_name.clone(),
                Arc::clone(&tool_clone),
                args,
            )
        })
    }
}
