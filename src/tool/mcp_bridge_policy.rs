//! Runtime approval preflight for MCP subprocess spawning.

use super::ToolResult;
use serde_json::Value;

pub(super) async fn blocked(
    invocation: &Value,
    command: &str,
    args: &[&str],
) -> Option<ToolResult> {
    if which::which(command).is_err() {
        return Some(ToolResult::error(format!("MCP executable not found: {command}")));
    }
    if invocation.get("approval_id").and_then(Value::as_str).is_some()
        && let Err(error) = crate::mcp::subprocess_policy::sandbox::preflight(
            command, args, crate::tool::network_access::allowed_for(invocation),
        ).await
    {
        return Some(ToolResult::error(format!("MCP sandbox preflight failed: {error}")));
    }
    if approved(invocation).await {
        return None;
    }
    crate::runtime_policy::evaluate_tool_invocation("mcp", invocation).await
}

async fn approved(args: &Value) -> bool {
    crate::runtime_policy::invocation::evaluate_approved_tool_invocation("mcp", args).await
        || crate::runtime_policy::invocation::evaluate_approved_tool_invocation(
            "mcp_bridge",
            args,
        )
        .await
}

#[cfg(test)]
#[path = "mcp_bridge_policy_test_env.rs"]
mod test_env;
#[cfg(test)]
#[path = "mcp_bridge_policy_tests.rs"]
mod tests;