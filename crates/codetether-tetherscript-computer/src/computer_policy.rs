//! Evaluate host computer-use policy using the current or temporary runtime.
use crate::Host;

use codetether_tool_core::ToolResult;
use tetherscript::value::Value;

pub(super) fn blocked<H: Host>(payload: &Value) -> Result<Option<ToolResult>, String> {
    let args = codetether_tetherscript_values::tetherscript_to_json(payload);
    match tokio::runtime::Handle::try_current() {
        Ok(handle) => handle.block_on(check::<H>(args)),
        Err(_) => runtime_check::<H>(args),
    }
}

async fn check<H: Host>(args: serde_json::Value) -> Result<Option<ToolResult>, String> {
    Ok(H::evaluate_tool_invocation("computer_use", &args).await)
}

fn runtime_check<H: Host>(args: serde_json::Value) -> Result<Option<ToolResult>, String> {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| format!("computer policy runtime failed: {e}"))?
        .block_on(check::<H>(args))
}
