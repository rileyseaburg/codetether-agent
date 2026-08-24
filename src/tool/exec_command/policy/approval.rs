//! Approval authority permitted for each exec sandbox mode.

use serde_json::Value;

pub(super) fn escalated(args: &Value) -> bool {
    args.get("sandbox_permissions").and_then(Value::as_str) == Some("require_escalated")
}

pub(super) fn exact(args: &Value) -> bool {
    crate::runtime_policy::approved_receipt("exec_command", args)
}