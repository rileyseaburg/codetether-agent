//! Reusable approval checks for execution backends.

use serde_json::Value;

pub fn approved_invocation(tool_name: &str, args: &Value) -> bool {
    let scope = super::invocation_scope::for_tool(tool_name, args);
    allowed(tool_name, args, scope.action, &scope.resource)
}

pub fn approved_or_session_command(tool_name: &str, args: &Value) -> bool {
    approved_invocation(tool_name, args) || super::session_command::approved(tool_name, args)
}

pub(super) fn allowed(tool_name: &str, args: &Value, action: &str, resource: &str) -> bool {
    crate::approval::session_grants::allowed(tool_name, action, resource)
        || super::approval::verified(args, tool_name, action, resource)
}
