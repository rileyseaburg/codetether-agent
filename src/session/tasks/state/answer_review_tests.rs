//! Exercise the question → No → follow-up → Yes state machine.

use super::super::answer_review_test_support::{decision, held};
use crate::session::tasks::{AnswerReviewAction as Action, GoalStatus};

#[test]
fn answer_review_only_acceptance_of_a_delivered_answer_resumes() {
    let mut state = held();
    state.apply(&decision("review", Action::Satisfied));
    assert_eq!(state.goal.as_ref().unwrap().status, GoalStatus::Paused);
    state.apply(&decision("review", Action::Answered));
    assert!(state.answer_review.as_ref().unwrap().ready);
    state.apply(&decision("review", Action::Unsatisfied));
    assert!(!state.answer_review.as_ref().unwrap().ready);
    state.apply(&decision("review", Action::Satisfied));
    assert!(state.answer_review.is_some());
    state.apply(&decision(
        "followup",
        Action::Begin {
            question: "More detail?".into(),
        },
    ));
    state.apply(&decision("followup", Action::Answered));
    state.apply(&decision("review", Action::Satisfied));
    assert_eq!(state.goal.as_ref().unwrap().status, GoalStatus::Paused);
    state.apply(&decision("followup", Action::Satisfied));
    assert_eq!(state.goal.as_ref().unwrap().status, GoalStatus::Active);
    assert!(state.answer_review.is_none());
}

#[path = "answer_review_continue_tests.rs"]
mod continuation;
#[path = "answer_review_persistence_tests.rs"]
mod persistence;
#[path = "answer_review_replacement_tests.rs"]
mod replacement;
#[path = "answer_review_runtime_tests.rs"]
mod runtime;
