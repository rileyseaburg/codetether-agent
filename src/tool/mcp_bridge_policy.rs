//! Runtime approval preflight for MCP subprocess spawning.

use super::ToolResult;
use serde_json::{Value, json};

pub(super) async fn blocked(
    command: &str,
    args: &[&str],
    approval_id: Option<&str>,
) -> Option<ToolResult> {
    let args = policy_args(command, args, approval_id);
    if alias_approval_allows(&args).await {
        return None;
    }
    crate::runtime_policy::evaluate_tool_invocation("mcp", &args).await
}

fn policy_args(command: &str, args: &[&str], approval_id: Option<&str>) -> Value {
    let mut value = json!({ "command": rendered(command, args) });
    if let Some(id) = approval_id.filter(|id| !id.trim().is_empty()) {
        value["approval_id"] = json!(id);
    }
    value
}

fn rendered(command: &str, args: &[&str]) -> String {
    std::iter::once(command)
        .chain(args.iter().copied())
        .collect::<Vec<_>>()
        .join(" ")
}

async fn alias_approval_allows(args: &Value) -> bool {
    crate::runtime_policy::approved_invocation("mcp_bridge", args)
        && crate::runtime_policy::evaluate_tool_invocation("mcp_bridge", args)
            .await
            .is_none()
}

#[cfg(test)]
#[path = "mcp_bridge_policy_tests.rs"]
mod tests;
