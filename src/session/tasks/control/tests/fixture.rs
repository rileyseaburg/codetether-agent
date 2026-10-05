//! Isolated journal state used by native goal-control tests.

use crate::session::tasks::{GoalEdit, TaskEvent, TaskState};
use serde_json::json;

/// Declare a goal using the actual durable event encoding.
pub(super) fn declaration() -> TaskEvent {
    serde_json::from_value(json!({
        "kind": "goal_set", "at": "2026-01-01T00:00:00Z",
        "goal_id": "goal", "objective": "Keep the objective",
        "success_criteria": ["Tests pass"], "forbidden": ["No deployment"]
    }))
    .unwrap()
}

/// Build native events for an exhausted one-token budget with prior usage.
pub(super) fn events() -> Vec<TaskEvent> {
    vec![
        declaration(),
        serde_json::from_value(json!({
            "kind": "goal_runtime", "at": "2026-01-01T00:00:00Z",
            "goal_id": "goal", "token_budget": 1, "token_delta": 37,
            "elapsed_seconds": 4, "continuation_delta": 2
        }))
        .unwrap(),
    ]
}

/// Fold the fixture through the production replay logic.
pub(super) fn state() -> TaskState {
    TaskState::from_log(&events())
}

/// Capture the exact goal version the editing user observed.
pub(super) fn request(state: &TaskState, action: &str) -> GoalEdit {
    let goal = state.goal.as_ref().unwrap();
    serde_json::from_value(json!({
        "goalId": goal.id, "updatedAt": goal.last_updated_at, "action": action
    }))
    .unwrap()
}
