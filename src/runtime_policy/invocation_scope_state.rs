//! Effective process-wide state that changes command execution authority.

#[path = "invocation_scope_tetherscript.rs"]
mod tetherscript;

use serde_json::{Value, json};

pub(super) fn bind(tool_name: &str, args: &mut Value, original: &Value) {
    let allowed = crate::tool::network_access::allowed_for(original);
    if tool_name == "tetherscript_plugin" {
        tetherscript::bind(args, allowed);
        return;
    }
    if !matches!(tool_name, "bash" | "exec_command" | "mcp" | "mcp_bridge")
        && !crate::runtime_policy::network::governed(tool_name, args)
    {
        return;
    }
    if let Some(map) = args.as_object_mut() {
        map.insert(crate::tool::network_access::FIELD.into(), json!(allowed));
    }
}
