//! Decode pin requests and load the exact calling session.
use crate::PinSource;
use crate::logic::apply_pin;
use anyhow::Result;
use codetether_tool_core::ToolResult;
use serde_json::Value;
pub(super) async fn run<S: PinSource>(args: Value) -> Result<ToolResult> {
    let action = args["action"].as_str().unwrap_or("").to_lowercase();
    let idx = match args["turn_index"].as_u64() {
        Some(i) => i as usize,
        None => {
            return Ok(ToolResult::error(
                "`turn_index` must be a non-negative integer.",
            ));
        }
    };
    let mut session = match codetether_context_loader::load_calling_session::<S>(&args).await {
        Ok(Some(s)) => s,
        Ok(None) => return Ok(ToolResult::error("No active session.")),
        Err(e) => return Ok(ToolResult::error(&format!("Load failed: {e}"))),
    };
    match apply_pin(&mut session, idx, &action).await {
        Ok(msg) => Ok(ToolResult::success(msg)),
        Err(e) => Ok(ToolResult::error(&e.to_string())),
    }
}
