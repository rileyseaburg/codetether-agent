//! Decode browse aliases and required turn indices.
use crate::ContextBrowseAction;
use serde_json::Value;

/// Parse a tool-args [`Value`] into a typed [`ContextBrowseAction`].
///
/// Factored out so the parsing logic is unit-testable without routing
/// through the full [`codetether_tool_core::Tool`] trait machinery.
///
/// # Arguments
/// * `args` - JSON tool arguments containing an optional action and turn index.
/// # Returns
/// A list action by default, or the explicitly requested action.
/// # Errors
/// Returns an error for unknown actions or missing/noninteger show-turn indices.
/// # Examples
/// ```rust
/// let action=codetether_context_browse::parse_browse_action(&serde_json::json!({})).unwrap();
/// assert_eq!(action,codetether_context_browse::ContextBrowseAction::ListTurns);
/// ```
pub fn parse_browse_action(args: &Value) -> Result<ContextBrowseAction, String> {
    let action = args.get("action").and_then(Value::as_str).unwrap_or("list");
    match action {
        "list" | "list_turns" => Ok(ContextBrowseAction::ListTurns),
        "show" | "show_turn" => {
            let turn = args
                .get("turn")
                .and_then(Value::as_u64)
                .ok_or_else(|| "`turn` (integer) is required for show_turn".to_string())?
                as usize;
            Ok(ContextBrowseAction::ShowTurn { turn })
        }
        other => Err(format!("unknown action: {other}")),
    }
}
