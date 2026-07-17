//! Maps model-facing actions onto MCP JSON-RPC methods and parameters.

use anyhow::{Result, anyhow};
use serde_json::{Value, json};

pub(super) struct Invocation {
    pub(super) method: &'static str,
    pub(super) params: Value,
}

pub(super) fn parse(input: Value) -> Result<Invocation> {
    let action = input["action"]
        .as_str()
        .ok_or_else(|| anyhow!("missing action"))?;
    match action {
        "list_tools" => Ok(Invocation {
            method: "tools/list",
            params: json!({}),
        }),
        "call_tool" => call(input),
        _ => Err(anyhow!("unsupported application MCP action: {action}")),
    }
}

fn call(input: Value) -> Result<Invocation> {
    let name = input["tool_name"]
        .as_str()
        .ok_or_else(|| anyhow!("call_tool requires tool_name"))?;
    let arguments = input.get("arguments").cloned().unwrap_or_else(|| json!({}));
    if !arguments.is_object() {
        return Err(anyhow!("call_tool arguments must be an object"));
    }
    Ok(Invocation {
        method: "tools/call",
        params: json!({ "name": name, "arguments": arguments }),
    })
}
