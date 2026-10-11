//! Fail-closed budget authority checks for native and legacy model tools.

use crate::tool::ToolResult;
use serde_json::Value;

pub(crate) fn refusal(input: &Value) -> Option<ToolResult> {
    ["token_budget", "tokenBudget"]
        .iter()
        .any(|key| input.get(key).is_some())
        .then(|| {
            ToolResult::error(
                "GOAL_BUDGET_HUMAN_ONLY: only human TUI or authenticated API controls may set, change, or remove a goal budget",
            )
        })
}
