use super::support::{set, update};
use crate::session::tasks::{GoalStatus, TaskState};

#[test]
fn folds_accounting_and_budget_limit() {
    let state = TaskState::from_log(&[set("g1"), update("g1", None, 12, Some(10))]);
    let goal = state.goal.expect("goal");
    assert_eq!(goal.status, GoalStatus::BudgetLimited);
    assert_eq!(goal.tokens_used, 12);
    assert_eq!(goal.time_used_seconds, 7);
    assert_eq!(goal.turns_used, 2);
}

#[test]
fn ignores_updates_for_replaced_goal() {
    let state = TaskState::from_log(&[
        set("old"),
        set("current"),
        update("old", Some(GoalStatus::Complete), 5, None),
    ]);
    let goal = state.goal.expect("goal");
    assert_eq!(goal.id, "current");
    assert_eq!(goal.status, GoalStatus::Active);
    assert_eq!(goal.tokens_used, 0);
}

#[test]
fn runtime_event_keeps_flat_json_shape() {
    let event = update("g1", Some(GoalStatus::Complete), 5, None);
    let value = serde_json::to_value(&event).expect("serialize runtime event");
    assert_eq!(value["kind"], "goal_runtime");
    assert_eq!(value["goal_id"], "g1");
    serde_json::from_value::<crate::session::tasks::TaskEvent>(value)
        .expect("deserialize runtime event");
}
