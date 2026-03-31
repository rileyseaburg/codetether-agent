use super::Agent;
use crate::tool::{Tool, ToolResult};
use serde_json::Value;
use std::sync::Arc;

impl Agent {
    /// Execute a single tool from JSON string arguments.
    pub(super) async fn execute_tool(&self, name: &str, arguments: &str) -> ToolResult {
        match serde_json::from_str(arguments) {
            Ok(value) => self.execute_tool_value(name, value).await,
            Err(error) => ToolResult::error(format!("Failed to parse arguments: {error}")),
        }
    }

    pub(super) async fn execute_tool_value(&self, name: &str, arguments: Value) -> ToolResult {
        self.log_tool_permission(name);

        match self.tools.get(name) {
            Some(tool) => execute_registered_tool(tool, arguments).await,
            None => self.execute_invalid_tool(name, arguments).await,
        }
    }

    async fn execute_invalid_tool(&self, name: &str, arguments: Value) -> ToolResult {
        let available_tools = self.tools.list().iter().map(ToString::to_string).collect();
        let invalid_tool =
            crate::tool::invalid::InvalidTool::with_context(name.to_string(), available_tools);
        let args = serde_json::json!({
            "requested_tool": name,
            "args": arguments,
        });

        match invalid_tool.execute(args).await {
            Ok(result) => result,
            Err(error) => ToolResult::error(format!("Unknown tool: {name}. Error: {error}")),
        }
    }

    fn log_tool_permission(&self, name: &str) {
        if let Some(permission) = self.permissions.get(name) {
            tracing::debug!(
                agent = %self.info.name,
                tool = %name,
                permission = ?permission,
                "Checking tool permission"
            );
        }
    }

    /// Get a tool from the registry by name
    pub fn get_tool(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.tools.get(name)
    }

    /// Register a tool with the agent's tool registry
    pub fn register_tool(&mut self, tool: Arc<dyn Tool>) {
        self.tools.register(tool);
    }

    /// List all available tool IDs
    pub fn list_tools(&self) -> Vec<&str> {
        self.tools.list()
    }

    /// Check if a tool is available
    pub fn has_tool(&self, name: &str) -> bool {
        self.tools.get(name).is_some()
    }
}

async fn execute_registered_tool(tool: Arc<dyn Tool>, arguments: Value) -> ToolResult {
    match tool.execute(arguments).await {
        Ok(result) => result,
        Err(error) => ToolResult::error(format!("Tool execution failed: {error}")),
    }
}
