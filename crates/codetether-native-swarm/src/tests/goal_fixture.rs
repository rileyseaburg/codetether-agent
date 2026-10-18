//! Canonical goal journal used by delegated usage accounting.
use chrono::Utc;
use codetether_native_goals::{GoalSourceKind, TaskEvent, TaskLog};
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

pub(super) struct GoalSession {
    pub(super) id: String,
}
pub(super) async fn session() -> GoalSession {
    let session = GoalSession {
        id: uuid::Uuid::new_v4().to_string(),
    };
    TaskLog::for_session(&session.id)
        .unwrap()
        .append(&goal("goal"))
        .await
        .unwrap();
    session
}
