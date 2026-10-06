//! Live accounting cannot turn an accepted human edit into a false conflict.

use crate::session::tasks::{GoalEdit, GoalEdited, TaskEvent, TaskState};
use chrono::{Duration, Utc};
use serde_json::json;

#[test]
fn session_goal_user_edit_confirmation_survives_later_accounting() {
    let at = Utc::now();
    let declaration: TaskEvent = serde_json::from_value(json!({
        "kind":"goal_set", "at":at, "goal_id":"g", "objective":"Before",
        "success_criteria":[], "forbidden":[]
    }))
    .unwrap();
    let request: GoalEdit = serde_json::from_value(json!({
        "goalId":"g", "updatedAt":at, "action":"override", "objective":"After"
    }))
    .unwrap();
    let edited_at = at + Duration::seconds(1);
    let events = vec![
        declaration,
        TaskEvent::GoalEdited(GoalEdited {
            at: edited_at,
            request,
        }),
        serde_json::from_value(json!({
            "kind":"goal_runtime", "at":at + Duration::seconds(2), "goal_id":"g",
            "token_delta":13, "elapsed_seconds":1, "continuation_delta":0
        }))
        .unwrap(),
    ];
    assert!(super::edit(&events, edited_at, false));
    let goal = TaskState::from_log(&events).goal.unwrap();
    assert_eq!(goal.objective, "After");
    assert_eq!(goal.tokens_used, 13);
    assert_ne!(goal.last_updated_at, edited_at);
}
