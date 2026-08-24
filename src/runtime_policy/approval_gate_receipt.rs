//! Exact one-time receipt checks for high-risk approval paths.

use serde_json::Value;

pub(super) fn required(tool: &str, args: &Value) -> bool {
    tool == "exec_command"
        && args.get("sandbox_permissions").and_then(Value::as_str) == Some("require_escalated")
}

pub(super) fn allowed(
    tool: &str,
    args: &Value,
    action: &str,
    resource: &str,
    inspect: bool,
) -> bool {
    if !args
        .get("approval_id")
        .and_then(Value::as_str)
        .is_some_and(|id| !id.trim().is_empty())
    {
        return false;
    }
    if inspect {
        crate::runtime_policy::approval::inspected(args, tool, action, resource)
    } else {
        crate::runtime_policy::approval::verified(args, tool, action, resource)
    }
}
