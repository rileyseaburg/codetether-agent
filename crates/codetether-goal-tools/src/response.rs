//! Stable JSON rendering for model-visible goal tool results.

use codetether_goal_core::TaskState;
use codetether_tool_core::ToolResult;
use serde_json::{Value, json};

/// Format current goal state for a model tool response.
/// # Arguments
/// * `state` - Folded goal/task journal state.
/// # Returns
/// Successful result containing the stable goal JSON projection.
/// # Examples
/// ```
/// let result = codetether_goal_tools::response::result(&codetether_goal_core::TaskState::from_log(&[]));
/// assert!(result.success);
/// ```
pub fn result(state: &TaskState) -> ToolResult {
    let goal = state.goal.as_ref().map(value);
    ToolResult::success(serde_json::to_string_pretty(&json!({ "goal": goal })).unwrap_or_default())
}

fn value(goal: &codetether_goal_core::Goal) -> Value {
    let remaining = goal
        .token_budget
        .map(|limit| limit.saturating_sub(goal.tokens_used).max(0));
    json!({
        "goalId": goal.id, "objective": goal.objective, "status": goal.status.as_str(),
        "tokenBudget": goal.token_budget, "tokensUsed": goal.tokens_used,
        "remainingTokens": remaining, "timeUsedSeconds": goal.time_used_seconds,
        "turnsUsed": goal.turns_used, "setAt": goal.set_at, "updatedAt": goal.last_updated_at,
        "successCriteria": goal.success_criteria, "forbidden": goal.forbidden,
    })
}
