//! Clearing an exhausted budget preserves identity, constraints, and usage.

use super::fixture;
use crate::session::tasks::{GoalEdited, GoalStatus, TaskEvent, TaskState};
use chrono::Utc;

/// Removing a cap is distinct from resuming a stopped native goal.
#[test]
fn session_goal_controls_remove_budget_preserves_accounting() {
    let mut events = fixture::events();
    let before = fixture::state();
    let mut request = fixture::request(&before, "edit");
    request.token_budget = Some(None);
    super::super::validate::request(&before, &request).unwrap();
    events.push(TaskEvent::GoalEdited(GoalEdited {
        at: Utc::now(),
        request,
    }));
    let state = TaskState::from_log(&events);
    let goal = state.goal.as_ref().unwrap();
    assert_eq!(goal.token_budget, None);
    assert_eq!(goal.id, "goal");
    assert_eq!(goal.objective, "Keep the objective");
    assert_eq!(goal.success_criteria, ["Tests pass"]);
    assert_eq!(goal.forbidden, ["No deployment"]);
    assert_eq!(
        (goal.tokens_used, goal.time_used_seconds, goal.turns_used),
        (37, 4, 3)
    );
    assert_eq!(goal.status, GoalStatus::BudgetLimited);
    let request = fixture::request(&state, "resume");
    super::super::validate::request(&state, &request).unwrap();
    events.push(TaskEvent::GoalEdited(GoalEdited {
        at: Utc::now(),
        request,
    }));
    assert_eq!(
        TaskState::from_log(&events).goal.unwrap().status,
        GoalStatus::Active
    );
}
