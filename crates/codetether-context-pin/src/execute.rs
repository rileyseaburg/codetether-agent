//! Decode pin requests and load the exact calling session.
use crate::PinSource;
use crate::logic::apply_pin;
use anyhow::Result;
use codetether_tool_core::ToolResult;
use serde_json::Value;
pub(super) async fn run<S: PinSource>(args: Value) -> Result<ToolResult> {
    let mut session = match codetether_context_loader::load_calling_session::<S>(&args).await {
        Ok(Some(s)) => s,
        Ok(None) => return Ok(ToolResult::error("No active session.")),
        Err(e) => return Ok(ToolResult::error(&format!("Load failed: {e}"))),
    };
    apply_to_session(&mut session, args).await
}

/// Execute a pin action against an already resolved calling session.
/// # Arguments
/// `session` is the authorized target; `args` uses the context-pin tool schema.
/// # Returns
/// The same success or error result used by the registered tool entry point.
/// # Errors
/// Unexpected runtime errors propagate; invalid input and storage errors return
/// an unsuccessful tool result without approving an action.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_context_pin::PinSource>(session: &mut S) -> anyhow::Result<()> {
/// let result = codetether_context_pin::apply_to_session(session,
///     serde_json::json!({"action":"pin", "turn_index":0})).await?;
/// # Ok(()) }
/// ```
pub async fn apply_to_session<S: PinSource>(session: &mut S, args: Value) -> Result<ToolResult> {
    let action = args["action"].as_str().unwrap_or("").to_lowercase();
    let idx = match args["turn_index"]
        .as_u64()
        .and_then(|index| usize::try_from(index).ok())
    {
        Some(i) => i,
        None => {
            return Ok(ToolResult::error(
                "`turn_index` must be a non-negative integer.",
            ));
        }
    };
    let replacement = args["replacement_index"]
        .as_u64()
        .and_then(|index| usize::try_from(index).ok());
    let mut sources = vec![idx];
    match action.as_str() {
        "correct" | "supersede" => sources.extend(replacement),
        "pin" | "unpin" => {}
        _ => {
            return Ok(ToolResult::error(
                "`action` must be pin, unpin, supersede, or correct.",
            ));
        }
    }
    if let Err(error) = session.prepare_sources(&sources).await {
        return Ok(ToolResult::error(error.to_string()));
    }
    if action == "correct" {
        return Ok(match super::correct::apply(session, idx, &args).await {
            Ok(message) => ToolResult::success(message),
            Err(error) => ToolResult::error(error.to_string()),
        });
    }
    match apply_pin(session, idx, &action, replacement).await {
        Ok(msg) => Ok(ToolResult::success(msg)),
        Err(e) => Ok(ToolResult::error(&e.to_string())),
    }
}
