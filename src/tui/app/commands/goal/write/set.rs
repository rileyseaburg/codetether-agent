//! `/goal set` persistence.

use crate::session::Session;
use crate::session::tasks::{GoalSourceKind, TaskEvent, TaskLog};
use anyhow::{Result, anyhow};
use chrono::Utc;

pub(super) async fn run(session: &Session, objective: &str) -> Result<String> {
    if objective.is_empty() {
        return Err(anyhow!("usage: /goal set <objective>"));
    }
    TaskLog::for_session(&session.id)?
        .append(&TaskEvent::GoalSet {
            at: Utc::now(),
            goal_id: uuid::Uuid::new_v4().to_string(),
            objective: objective.into(),
            success_criteria: Vec::new(),
            forbidden: Vec::new(),
            source_session_id: session.id.clone(),
            source_turn_id: String::new(),
            source_text_hash: String::new(),
            source_kind: GoalSourceKind::UserProvided,
            confidence: 1.0,
        })
        .await?;
    Ok(format!("Goal set: {objective}"))
}
