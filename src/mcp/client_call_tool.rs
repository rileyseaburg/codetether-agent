//! Runtime-policy guard for remote MCP tool calls.

use super::{CallToolResult, McpClient, ToolContent};
use anyhow::Result;
use serde_json::Value;

impl McpClient {
    pub(super) async fn call_tool_checked(
        &self,
        name: &str,
        arguments: Value,
    ) -> Result<CallToolResult> {
        if let Some(blocked) = blocked(name, &arguments).await {
            return Ok(blocked);
        }
        let arguments = strip_approval_id(arguments);
        self.inner.connection().call_tool(name, arguments).await
    }
}

async fn blocked(name: &str, arguments: &Value) -> Option<CallToolResult> {
    if let Some(blocked) = crate::runtime_policy::evaluate_tool_invocation(name, &arguments).await {
        return Some(CallToolResult {
            content: vec![ToolContent::Text {
                text: blocked.output,
            }],
            is_error: true,
        });
    }
    None
}

fn strip_approval_id(mut arguments: Value) -> Value {
    if let Some(map) = arguments.as_object_mut() {
        map.remove("approval_id");
    }
    arguments
}
