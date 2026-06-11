//! Registered tool execution wrapper.

use crate::tool::{Tool, ToolResult};
use serde_json::Value;
use std::sync::Arc;

pub(super) async fn execute(tool: Arc<dyn Tool>, arguments: Value) -> ToolResult {
    let result = match tool.execute(arguments).await {
        Ok(result) => result,
        Err(error) => ToolResult::error(format!("Tool execution failed: {error}")),
    };
    result.truncate_to(crate::tool::tool_output_budget())
}
