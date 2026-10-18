//! Pure goal and answer-review event fixtures.
use chrono::Utc;
use codetether_native_goals::{AnswerReviewAction, AnswerReviewUpdate, GoalSourceKind, TaskEvent};
pub(super) fn goal(id: &str) -> TaskEvent {
    TaskEvent::GoalSet {
        at: Utc::now(),
        goal_id: id.into(),
        objective: "Finish the goal".into(),
        success_criteria: vec![],
        forbidden: vec![],
        source_session_id: "fixture".into(),
        source_turn_id: String::new(),
        source_text_hash: String::new(),
        source_kind: GoalSourceKind::UserProvided,
        confidence: 1.0,
    }
}

pub(super) fn decision(id: &str, action: AnswerReviewAction) -> TaskEvent {
    TaskEvent::AnswerReview(AnswerReviewUpdate {
        at: Utc::now(),
        goal_id: "goal".into(),
        review_id: id.into(),
        decision: action,
    })
}
