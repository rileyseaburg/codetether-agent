//! Invocation arguments used consistently by primary and alias policy checks.
use serde_json::{Value, json};

/// Build the policy arguments for an external MCP process.
///
/// # Arguments
/// * `command` — Executable name.
/// * `args` — Process arguments rendered in their existing order.
/// * `approval_id` — Optional nonblank approval receipt, preserved verbatim.
/// # Returns
/// Command text and, when present, the approval identifier.
/// # Examples
/// ```
/// let args = codetether_mcp_approval::subprocess::policy_args("npx", &["server"], None);
/// assert_eq!(args["command"], "npx server");
/// ```
pub fn policy_args(command: &str, args: &[&str], approval_id: Option<&str>) -> Value {
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
