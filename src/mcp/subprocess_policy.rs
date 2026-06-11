//! Runtime policy gate for spawning external MCP server processes.

use anyhow::{Result, bail};
use serde_json::json;

pub(super) async fn guard(command: &str, args: &[&str], approval_id: Option<&str>) -> Result<()> {
    let policy_args = policy_args(command, args, approval_id);
    let Some(blocked) = crate::runtime_policy::evaluate_tool_invocation("mcp", &policy_args).await
    else {
        return Ok(());
    };
    if alias_approval_allows(&policy_args).await {
        return Ok(());
    }
    bail!(blocked.output)
}

fn policy_args(command: &str, args: &[&str], approval_id: Option<&str>) -> serde_json::Value {
    let mut policy_args = json!({ "command": rendered(command, args) });
    if let Some(approval_id) = approval_id.filter(|id| !id.trim().is_empty()) {
        policy_args["approval_id"] = json!(approval_id);
    }
    policy_args
}

fn rendered(command: &str, args: &[&str]) -> String {
    std::iter::once(command)
        .chain(args.iter().copied())
        .collect::<Vec<_>>()
        .join(" ")
}

async fn alias_approval_allows(args: &serde_json::Value) -> bool {
    crate::runtime_policy::approved_invocation("mcp_bridge", args)
        && crate::runtime_policy::evaluate_tool_invocation("mcp_bridge", args)
            .await
            .is_none()
}

#[cfg(test)]
#[path = "subprocess_policy_tests.rs"]
mod tests;
