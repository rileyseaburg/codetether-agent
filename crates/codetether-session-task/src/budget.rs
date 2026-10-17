//! Fail-closed budget authority checks for native and legacy model tools.

use codetether_tool_core::ToolResult;
use serde_json::Value;

/// Reject model-supplied goal budgets before parsing or session lookup.
///
/// # Arguments
/// * `input` — Raw model-supplied tool arguments.
///
/// # Returns
/// A tool error when either budget key is present, including a null value.
///
/// # Examples
/// ```
/// assert!(codetether_session_task::budget_refusal(&serde_json::json!({"token_budget":1})).is_some());
/// ```
pub fn refusal(input: &Value) -> Option<ToolResult> {
    ["token_budget", "tokenBudget"]
        .iter()
        .any(|key| input.get(key).is_some())
        .then(|| {
            ToolResult::error(
                "GOAL_BUDGET_HUMAN_ONLY: only human TUI or authenticated API controls may set, change, or remove a goal budget",
            )
        })
}
