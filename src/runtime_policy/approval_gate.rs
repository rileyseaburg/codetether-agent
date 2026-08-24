//! Reusable approval checks for execution backends.

#[path = "approval_gate_receipt.rs"]
mod receipt;

use serde_json::Value;

pub fn approved_invocation(tool_name: &str, args: &Value) -> bool {
    let scope = super::invocation_scope::for_tool(tool_name, args);
    allowed(tool_name, args, scope.action, &scope.resource)
}

pub fn approved_receipt(tool_name: &str, args: &Value) -> bool {
    let scope = super::invocation_scope::for_tool(tool_name, args);
    receipt::allowed(tool_name, args, scope.action, &scope.resource, false)
}

pub fn approved_or_session_command(tool_name: &str, args: &Value) -> bool {
    approved_invocation(tool_name, args) || super::session_command::approved(tool_name, args)
}

pub(super) fn allowed(tool_name: &str, args: &Value, action: &str, resource: &str) -> bool {
    if receipt::required(tool_name, args) {
        return receipt::allowed(tool_name, args, action, resource, false);
    }
    if args
        .get("approval_id")
        .and_then(Value::as_str)
        .is_some_and(|value| !value.trim().is_empty())
    {
        return super::approval::verified(args, tool_name, action, resource);
    }
    crate::approval::session_grants::allowed_scoped(
        tool_name,
        action,
        resource,
        args.get("__ct_session_id").and_then(Value::as_str),
    )
}

pub(super) fn review_allowed(tool_name: &str, args: &Value, action: &str, resource: &str) -> bool {
    if receipt::required(tool_name, args) {
        return receipt::allowed(tool_name, args, action, resource, true);
    }
    if args.get("approval_id").and_then(Value::as_str).is_some() {
        return super::approval::inspected(args, tool_name, action, resource);
    }
    crate::approval::session_grants::allowed_scoped(
        tool_name,
        action,
        resource,
        args.get("__ct_session_id").and_then(Value::as_str),
    )
}
