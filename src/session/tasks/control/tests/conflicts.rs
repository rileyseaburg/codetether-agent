//! Invalid, stale, or terminal claims cannot bypass native goal governance.

use super::fixture;
use crate::session::tasks::{GoalEdited, TaskEvent, TaskState};
use chrono::Utc;
use serde_json::json;

/// Reject stale versions and never accept completion as a user-edit action.
#[test]
fn session_goal_controls_reject_stale_or_terminal_requests() {
    let state = fixture::state();
    let mut request = fixture::request(&state, "edit");
    request.goal_id = "other-goal".into();
    assert!(super::super::validate::request(&state, &request).is_err());
    let mut events = fixture::events();
    events.push(TaskEvent::GoalEdited(GoalEdited {
        at: Utc::now(),
        request,
    }));
    assert_eq!(TaskState::from_log(&events).goal.unwrap().id, "goal");
    let request = fixture::request(&state, "resume");
    assert!(super::super::validate::request(&state, &request).is_err());
    for action in ["complete", "done", "blocked"] {
        let mut value = serde_json::to_value(&request).unwrap();
        value["action"] = json!(action);
        assert!(serde_json::from_value::<crate::session::tasks::GoalEdit>(value).is_err());
    }
    let mut request = fixture::request(&state, "edit");
    request.updated_at = Utc::now();
    assert!(super::super::validate::request(&state, &request).is_err());
}
