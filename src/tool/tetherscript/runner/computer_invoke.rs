use crate::tool::Tool;
use crate::tool::computer_use::ComputerUseTool;
use crate::tool::computer_use::input::ComputerUseInput;
use crate::tool::tetherscript::convert::{json_to_tetherscript, tetherscript_to_json};
use tetherscript::value::Value;

use super::{computer::ComputerAuthority, computer_value};

/// Invoke an existing `computer_use` action from a TetherScript computer grant.
pub fn invoke(auth: &ComputerAuthority, method: &str, args: &[Value]) -> Result<Value, String> {
    let (payload, scope) = super::computer_payload::prepare(method, args)?;
    require_scope(auth, scope)?;
    let json = tetherscript_to_json(&payload);
    serde_json::from_value::<ComputerUseInput>(json.clone())
        .map_err(|e| format!("computer.{method}: invalid computer_use payload: {e}"))?;
    let result = run_dispatch(json)?;
    let json = serde_json::to_value(result)
        .map_err(|e| format!("computer.{method}: encode result failed: {e}"))?;
    Ok(json_to_tetherscript(json))
}

fn require_scope(auth: &ComputerAuthority, scope: &str) -> Result<(), String> {
    computer_value::scopes(auth)
        .contains(scope)
        .then_some(())
        .ok_or_else(|| format!("computer: scope `{scope}` not granted"))
}

fn run_dispatch(args: serde_json::Value) -> Result<crate::tool::ToolResult, String> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| format!("computer: runtime create failed: {e}"))?;
    rt.block_on(ComputerUseTool::new().execute(args))
        .map_err(|e| format!("computer: computer_use dispatch failed: {e}"))
}
