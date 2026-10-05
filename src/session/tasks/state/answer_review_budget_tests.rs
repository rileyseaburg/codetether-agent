//! User acceptance restores safety limits, not unconditional active status.

use crate::session::tasks::answer_review_test_support::{decision, held};
use crate::session::tasks::{AnswerReviewAction as Action, GoalStatus};

#[test]
fn answer_review_yes_preserves_budget_limit() {
    let mut state = held();
    state.apply(&super::update(GoalStatus::Active, 5, Some(5)));
    state.apply(&decision("review", Action::Answered));
    state.apply(&decision("review", Action::Satisfied));
    assert_eq!(state.goal.unwrap().status, GoalStatus::BudgetLimited);
    assert!(state.answer_review.is_none());
}

#[test]
fn answer_review_followup_preserves_original_resume_status() {
    let mut state = held();
    state.apply(&decision("review", Action::Answered));
    state.apply(&decision("review", Action::Unsatisfied));
    state.apply(&decision(
        "second",
        Action::Begin {
            question: "Why again?".into(),
        },
    ));
    state.apply(&decision("second", Action::Answered));
    state.apply(&decision("second", Action::Satisfied));
    assert_eq!(state.goal.unwrap().status, GoalStatus::Active);
}
