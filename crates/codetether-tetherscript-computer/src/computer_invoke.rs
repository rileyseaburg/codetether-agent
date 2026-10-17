//! Enforce scope and origin before policy, parsing and backend dispatch.
use crate::Host;
use codetether_tetherscript_values::{json_to_tetherscript, tetherscript_to_json};
use tetherscript::value::Value;

use super::{computer::ComputerAuthority, computer_value};

/// Invoke an existing `computer_use` action from a TetherScript computer grant.
pub fn invoke<H: Host>(
    auth: &ComputerAuthority<H>,
    method: &str,
    args: &[Value],
) -> Result<Value, String> {
    let (payload, scope) = super::computer_payload::prepare(method, args)?;
    require_scope(auth, method, scope)?;
    super::computer_origin::require(auth, method, &payload)?;
    if let Some(blocked) = super::computer_policy::blocked::<H>(&payload)? {
        return tool_result(blocked, method);
    }
    let input = input_from_payload::<H>(method, &payload)?;
    let result = super::computer_dispatch::run_dispatch::<H>(&input)?;
    tool_result(result, method)
}

fn require_scope<H: Host>(
    auth: &ComputerAuthority<H>,
    method: &str,
    scope: &str,
) -> Result<(), String> {
    computer_value::scopes(auth)
        .contains(scope)
        .then_some(())
        .ok_or_else(|| format!("computer.{method}: scope `{scope}` not granted"))
}

fn input_from_payload<H: Host>(method: &str, payload: &Value) -> Result<H::Input, String> {
    let json = tetherscript_to_json(payload);
    serde_json::from_value::<H::Input>(json)
        .map_err(|e| format!("computer.{method}: invalid computer_use payload: {e}"))
}

fn tool_result(result: codetether_tool_core::ToolResult, method: &str) -> Result<Value, String> {
    let json = serde_json::to_value(result)
        .map_err(|e| format!("computer.{method}: encode result failed: {e}"))?;
    Ok(json_to_tetherscript(json))
}
