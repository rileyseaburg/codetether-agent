//! Runtime-policy guard for remote MCP tool calls.

#[path = "client_call_tool_args.rs"]
mod args;

use super::{CallToolParams, CallToolResult, McpClient, ToolContent};
use anyhow::Result;
use serde_json::Value;

impl McpClient {
    pub(super) async fn call_tool_checked(
        &self,
        name: &str,
        arguments: Value,
    ) -> Result<CallToolResult> {
        let policy_name = format!("mcp:{}:{name}", self.approval_identity().await);
        if let Some(blocked) = blocked(&policy_name, &arguments).await {
            return Ok(blocked);
        }
        self.call_tool_authorized(name, arguments).await
    }

    pub(crate) async fn call_tool_authorized(
        &self,
        name: &str,
        arguments: Value,
    ) -> Result<CallToolResult> {
        let params = CallToolParams {
            name: name.to_string(),
            arguments: args::sanitize(arguments),
        };
        let response = self
            .request("tools/call", Some(serde_json::to_value(&params)?))
            .await?;
        Ok(serde_json::from_value(response)?)
    }
}

async fn blocked(name: &str, arguments: &Value) -> Option<CallToolResult> {
    if let Some(blocked) = crate::runtime_policy::evaluate_tool_invocation(name, arguments).await {
        return Some(policy_error(blocked.output));
    }
    if crate::runtime_policy::approval::self_verifying(name)
        && let Err(error) = crate::approval::use_once::claim(name, arguments)
    {
        return Some(policy_error(format!("approval claim failed: {error}")));
    }
    None
}

fn policy_error(text: String) -> CallToolResult {
    CallToolResult {
        content: vec![ToolContent::Text { text }],
        is_error: true,
    }
}
