//! Tool execution paths for agents.
//!
//! This module handles normal tool execution and invalid-tool fallback
//! responses without mixing registry access helpers into the same file.
//!
//! # Examples
//!
//! ```ignore
//! let result = agent.execute_tool("bash", "{\"command\":\"pwd\"}").await;
//! ```

use crate::agent::Agent;
use crate::tool::{Tool, ToolResult};
use serde_json::Value;
use std::sync::Arc;

impl Agent {
    /// Executes a tool from its serialized JSON argument string.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let result = agent.execute_tool("bash", "{\"command\":\"pwd\"}").await;
    /// ```
    pub(crate) async fn execute_tool(&self, name: &str, arguments: &str) -> ToolResult {
        match serde_json::from_str(arguments) {
            Ok(value) => self.execute_tool_value(name, value).await,
            Err(error) => ToolResult::error(format!("Failed to parse arguments: {error}")),
        }
    }

    /// Executes a tool from an already parsed JSON value.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let result = agent.execute_tool_value("bash", serde_json::json!({"command":"pwd"})).await;
    /// ```
    pub(crate) async fn execute_tool_value(&self, name: &str, arguments: Value) -> ToolResult {
        self.log_tool_permission(name);
        match self.tools.get(name) {
            Some(tool) => execute_registered_tool(tool, arguments).await,
            None => execute_invalid_tool(self, name, arguments).await,
        }
    }

    fn log_tool_permission(&self, name: &str) {
        if let Some(permission) = self.permissions.get(name) {
            tracing::debug!(agent = %self.info.name, tool = %name, permission = ?permission, "Checking tool permission");
        }
    }
}

async fn execute_registered_tool(tool: Arc<dyn Tool>, arguments: Value) -> ToolResult {
    match tool.execute(arguments).await {
        Ok(result) => result,
        Err(error) => ToolResult::error(format!("Tool execution failed: {error}")),
    }
}

async fn execute_invalid_tool(agent: &Agent, name: &str, arguments: Value) -> ToolResult {
    let available_tools = agent.tools.list().iter().map(ToString::to_string).collect();
    let invalid_tool =
        crate::tool::invalid::InvalidTool::with_context(name.to_string(), available_tools);
    let args = serde_json::json!({ "requested_tool": name, "args": arguments });
    match invalid_tool.execute(args).await {
        Ok(result) => result,
        Err(error) => ToolResult::error(format!("Unknown tool: {name}. Error: {error}")),
    }
}
