use crate::tool::computer_use::{input::ComputerUseInput, platform};
use crate::tool::tetherscript::convert::{json_to_tetherscript, tetherscript_to_json};
use tetherscript::value::Value;

use super::{computer::ComputerAuthority, computer_value};

/// Invoke an existing `computer_use` action from a TetherScript computer grant.
pub fn invoke(auth: &ComputerAuthority, method: &str, args: &[Value]) -> Result<Value, String> {
    let (payload, scope) = super::computer_payload::prepare(method, args)?;
    require_scope(auth, method, scope)?;
    super::computer_origin::require(auth, method, &payload)?;
    let input = input_from_payload(method, &payload)?;
    let result = run_dispatch(&input)?;
    let json = serde_json::to_value(result)
        .map_err(|e| format!("computer.{method}: encode result failed: {e}"))?;
    Ok(json_to_tetherscript(json))
}

fn require_scope(auth: &ComputerAuthority, method: &str, scope: &str) -> Result<(), String> {
    computer_value::scopes(auth)
        .contains(scope)
        .then_some(())
        .ok_or_else(|| format!("computer.{method}: scope `{scope}` not granted"))
}

fn input_from_payload(method: &str, payload: &Value) -> Result<ComputerUseInput, String> {
    let json = tetherscript_to_json(payload);
    serde_json::from_value::<ComputerUseInput>(json)
        .map_err(|e| format!("computer.{method}: invalid computer_use payload: {e}"))
}

fn run_dispatch(input: &ComputerUseInput) -> Result<crate::tool::ToolResult, String> {
    match tokio::runtime::Handle::try_current() {
        Ok(handle) => handle.block_on(platform::dispatch(input)),
        Err(_) => runtime_dispatch(input),
    }
    .map_err(|e| format!("computer: computer_use dispatch failed: {e}"))
}

fn runtime_dispatch(input: &ComputerUseInput) -> anyhow::Result<crate::tool::ToolResult> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?
        .block_on(platform::dispatch(input))
}
