//! Runtime approval preflight for MCP subprocess spawning.

use crate::Host;
use codetether_tool_core::ToolResult;
use serde_json::{Value, json};

/// Evaluate the bridge command using canonical and compatibility-alias approval.
///
/// # Arguments
/// * `command` - Executable requested by the caller.
/// * `args` - Executable arguments in their original order.
/// * `approval_id` - Optional receipt to include in policy arguments.
/// # Returns
/// A blocking tool result, or `None` when the invocation may proceed.
/// # Examples
/// ```rust,no_run
/// # async fn example<H:codetether_mcp_bridge::Host>() {
/// let _decision=codetether_mcp_bridge::policy::blocked::<H>("server",&[],None).await;
/// # }
/// ```
pub async fn blocked<H: Host>(
    command: &str,
    args: &[&str],
    approval_id: Option<&str>,
) -> Option<ToolResult> {
    let args = policy_args(command, args, approval_id);
    if alias_approval_allows::<H>(&args).await {
        return None;
    }
    H::evaluate_tool_invocation("mcp", &args).await
}

/// Render the executable and arguments into the shared approval payload.
///
/// # Arguments
/// * `command` - Executable name.
/// * `args` - Ordered executable arguments.
/// * `approval_id` - Optional nonblank approval receipt.
/// # Returns
/// A command object with a receipt when supplied.
/// # Examples
/// ```rust
/// let args=codetether_mcp_bridge::policy::policy_args("server",&["--local"],Some("id"));
/// assert_eq!(args["command"],"server --local");
/// assert_eq!(args["approval_id"],"id");
/// ```
pub fn policy_args(command: &str, args: &[&str], approval_id: Option<&str>) -> Value {
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

async fn alias_approval_allows<H: Host>(args: &Value) -> bool {
    H::approved_invocation("mcp_bridge", args)
        && H::evaluate_tool_invocation("mcp_bridge", args)
            .await
            .is_none()
}
