use crate::session::tasks::{GoalRuntimeUpdate, GoalSourceKind, GoalStatus, TaskEvent};
use chrono::Utc;

pub(super) fn set(id: &str) -> TaskEvent {
    TaskEvent::GoalSet {
        at: Utc::now(),
        goal_id: id.into(),
        objective: "ship parity".into(),
        success_criteria: vec!["tests pass".into()],
        forbidden: Vec::new(),
        source_session_id: "s1".into(),
        source_turn_id: String::new(),
        source_text_hash: String::new(),
        source_kind: GoalSourceKind::UserProvided,
        confidence: 1.0,
    }
}

pub(super) fn update(
    id: &str,
    status: Option<GoalStatus>,
    tokens: i64,
    budget: Option<i64>,
) -> TaskEvent {
    TaskEvent::GoalRuntime(GoalRuntimeUpdate {
        at: Utc::now(),
        goal_id: id.into(),
        objective: None,
        status,
        token_budget: budget,
        token_delta: tokens,
        elapsed_seconds: 7,
        continuation_delta: 1,
    })
}
