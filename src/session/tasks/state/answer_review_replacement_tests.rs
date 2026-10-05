//! Replay also rejects a non-user replacement already in flight at hold time.

use crate::session::tasks::answer_review_test_support::{goal, held};
use crate::session::tasks::{GoalSourceKind, GoalStatus, TaskEvent};

#[test]
fn answer_review_non_user_replacement_cannot_drop_hold() {
    for source in [
        GoalSourceKind::Inferred,
        GoalSourceKind::RawTurn,
        GoalSourceKind::RecallSummary,
        GoalSourceKind::Memory,
    ] {
        let mut state = held();
        let mut replacement = goal("model-replacement");
        if let TaskEvent::GoalSet { source_kind, .. } = &mut replacement {
            *source_kind = source;
        }
        state.apply(&replacement);
        let goal = state.goal.unwrap();
        assert_eq!(goal.id, "goal");
        assert_eq!(goal.status, GoalStatus::Paused);
        assert_eq!(state.answer_review.unwrap().id, "review");
    }
}
