//! Shared nonsecret task-log fixtures for answer-review tests.

use super::{TaskEvent, TaskState};
use crate::session::tasks::{AnswerReviewAction, AnswerReviewUpdate, GoalSourceKind};
use chrono::Utc;

pub(crate) fn goal(id: &str) -> TaskEvent {
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

pub(crate) fn decision(id: &str, action: AnswerReviewAction) -> TaskEvent {
    TaskEvent::AnswerReview(AnswerReviewUpdate {
        at: Utc::now(),
        goal_id: "goal".into(),
        review_id: id.into(),
        decision: action,
    })
}

pub(crate) fn held() -> TaskState {
    TaskState::from_log(&[
        goal("goal"),
        decision(
            "review",
            AnswerReviewAction::Begin {
                question: "Why?".into(),
            },
        ),
    ])
}

pub(crate) async fn session() -> crate::session::Session {
    let session = crate::session::Session::new().await.unwrap();
    crate::session::tasks::TaskLog::for_session(&session.id)
        .unwrap()
        .append(&goal("goal"))
        .await
        .unwrap();
    session
}
