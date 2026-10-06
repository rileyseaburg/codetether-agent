//! Native goal projection shared by the goal-control HTTP handlers.

use crate::session::tasks::TaskState;
use serde_json::{Value, json};

/// Return authoritative state; no UI request fields are echoed as success.
pub(super) fn value(session_id: &str, state: &TaskState) -> Value {
    let goal = state.goal.as_ref().map(|goal| {
        json!({
            "id": goal.id, "objective": goal.objective,
            "successCriteria": goal.success_criteria,
            "forbidden": goal.forbidden, "status": goal.status.as_str(),
            "tokenBudget": goal.token_budget, "tokensUsed": goal.tokens_used,
            "timeUsedSeconds": goal.time_used_seconds,
            "turnsUsed": goal.turns_used, "updatedAt": goal.last_updated_at,
        })
    });
    json!({"sessionId": session_id, "state": "ready",
        "kind": "session_goal", "tasksManagedSeparately": true,
        "goal": goal, "editable": true,
        "answerReviewPending": state.answer_review.is_some(),
        "humanActions": ["edit", "pause", "resume", "clear", "override", "force_complete"]
    })
}
