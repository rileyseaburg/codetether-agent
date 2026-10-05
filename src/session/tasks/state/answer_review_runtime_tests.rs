//! Runtime status changes and tool approval modes cannot release a user hold.

use crate::session::tasks::answer_review_test_support::held;
use crate::session::tasks::{GoalRuntimeUpdate, GoalStatus, TaskEvent};
use chrono::Utc;

pub(super) fn update(status: GoalStatus, tokens: i64, budget: Option<i64>) -> TaskEvent {
    TaskEvent::GoalRuntime(GoalRuntimeUpdate {
        at: Utc::now(),
        goal_id: "goal".into(),
        objective: None,
        status: Some(status),
        token_budget: budget,
        token_delta: tokens,
        elapsed_seconds: 3,
        continuation_delta: 1,
    })
}

#[test]
fn answer_review_runtime_cannot_resume_or_complete_held_goal() {
    let mut state = held();
    for status in [
        GoalStatus::Active,
        GoalStatus::Complete,
        GoalStatus::Blocked,
    ] {
        state.apply(&update(status, 2, None));
        assert_eq!(state.goal.as_ref().unwrap().status, GoalStatus::Paused);
        assert!(state.answer_review.is_some());
    }
    assert_eq!(state.goal.unwrap().tokens_used, 6);
}

#[test]
fn answer_review_old_goal_decision_cannot_hold_replacement() {
    use crate::session::tasks::AnswerReviewAction;
    use crate::session::tasks::answer_review_test_support::{decision, goal};
    let mut state = held();
    state.apply(&goal("replacement"));
    state.apply(&decision("review", AnswerReviewAction::Satisfied));
    assert!(state.answer_review.is_none());
    assert_eq!(state.goal.unwrap().status, GoalStatus::Active);
}

#[path = "answer_review_budget_tests.rs"]
mod budget;
