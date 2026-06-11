//! Runtime-policy bridge for TetherScript computer grants.

use crate::tool::ToolResult;
use tetherscript::value::Value;

pub(super) fn blocked(payload: &Value) -> Result<Option<ToolResult>, String> {
    let args = crate::tool::tetherscript::convert::tetherscript_to_json(payload);
    match tokio::runtime::Handle::try_current() {
        Ok(handle) => handle.block_on(check(args)),
        Err(_) => runtime_check(args),
    }
}

async fn check(args: serde_json::Value) -> Result<Option<ToolResult>, String> {
    Ok(crate::runtime_policy::evaluate_tool_invocation("computer_use", &args).await)
}

fn runtime_check(args: serde_json::Value) -> Result<Option<ToolResult>, String> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| format!("computer policy runtime failed: {e}"))?
        .block_on(check(args))
}
