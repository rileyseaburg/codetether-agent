//! Explicit continuation is identity scoped, durable, and budget constrained.
use crate::session::tasks::answer_review_test_support::{decision, held};
use crate::session::tasks::{AnswerReviewAction as Action, GoalStatus};

#[test]
fn answer_review_continuation_ignores_stale_ids() {
    let mut state = held();
    state.apply(&decision("old", Action::ResumeRequested));
    assert!(state.answer_review.is_some());
    let event = decision("review", Action::ResumeRequested);
    let json = serde_json::to_string(&event).unwrap();
    state.apply(&serde_json::from_str(&json).unwrap());
    assert!(state.answer_review.is_none());
    assert_eq!(state.goal.unwrap().status, GoalStatus::Active);
}

#[test]
fn answer_review_continuation_preserves_budget_and_paused_status() {
    for status in [GoalStatus::Active, GoalStatus::Paused] {
        let mut state = held();
        let goal = state.goal.as_mut().unwrap();
        goal.token_budget = Some(5);
        goal.tokens_used = 5;
        state.answer_review.as_mut().unwrap().resume_status = status;
        state.apply(&decision("review", Action::ResumeRequested));
        assert!(state.answer_review.is_none());
        let expected = if status.is_active() {
            GoalStatus::BudgetLimited
        } else {
            GoalStatus::Paused
        };
        assert_eq!(state.goal.unwrap().status, expected);
    }
}
