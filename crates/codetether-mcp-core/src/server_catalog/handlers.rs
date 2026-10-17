//! Synchronous handlers stored by the MCP server catalog.
use super::*;

/// Synchronous tool handler producing protocol content or an execution error.
pub type McpToolHandler = Arc<dyn Fn(Value) -> Result<CallToolResult> + Send + Sync>;

/// Synchronous resource handler receiving the requested URI.
pub type McpResourceHandler = Arc<dyn Fn(String) -> Result<ReadResourceResult> + Send + Sync>;

/// Synchronous prompt handler receiving protocol arguments.
pub type McpPromptHandler = Arc<dyn Fn(Value) -> Result<GetPromptResult> + Send + Sync>;
