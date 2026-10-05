//! Budget edit encoding distinguishes preservation, removal, and a new limit.

use super::fixture;
use crate::session::tasks::{GoalEdit, TaskEvent};
use serde_json::json;

/// Explicit null remains explicit through durable serialization and replay.
#[test]
fn session_goal_controls_budget_wire_contract() {
    let mut value = serde_json::to_value(fixture::request(&fixture::state(), "edit")).unwrap();
    assert!(value.get("tokenBudget").is_none());
    assert!(
        serde_json::from_value::<GoalEdit>(value.clone())
            .unwrap()
            .token_budget
            .is_none()
    );
    value["tokenBudget"] = json!(null);
    let removed: GoalEdit = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(removed.token_budget, Some(None));
    assert!(serde_json::to_value(removed).unwrap()["tokenBudget"].is_null());
    value["tokenBudget"] = json!(100);
    assert_eq!(
        serde_json::from_value::<GoalEdit>(value)
            .unwrap()
            .token_budget,
        Some(Some(100))
    );
}

/// Extracting the drift payload must not change the old native JSON shape.
#[test]
fn session_goal_controls_preserve_drift_wire_contract() {
    let value = json!({
        "kind": "drift_detected", "at": "2026-01-01T00:00:00Z",
        "tool_calls_since_reaffirm": 3, "errors_since_reaffirm": 1
    });
    let event: TaskEvent = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(serde_json::to_value(event).unwrap(), value);
}
