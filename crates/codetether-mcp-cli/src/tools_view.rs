//! Shared access to display fields without changing wire serialization.
use codetether_mcp_core::types::{McpTool, ToolMetadata};

pub(crate) trait View {
    fn name(&self) -> &str;
    fn description(&self) -> Option<&str>;
}

impl View for McpTool {
    fn name(&self) -> &str {
        &self.name
    }
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

impl View for ToolMetadata {
    fn name(&self) -> &str {
        &self.name
    }
    fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}
