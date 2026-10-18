//! `/goal set` persistence.
use crate::Host;

use anyhow::{Result, anyhow};
use chrono::Utc;
use codetether_goal_core::{GoalSourceKind, TaskEvent};

pub(super) async fn run<H: Host>(session_id: &str, objective: &str) -> Result<String> {
    if objective.is_empty() {
        return Err(anyhow!("usage: /goal set <objective>"));
    }
    H::log(session_id)?
        .append(&TaskEvent::GoalSet {
            at: Utc::now(),
            goal_id: uuid::Uuid::new_v4().to_string(),
            objective: objective.into(),
            success_criteria: Vec::new(),
            forbidden: Vec::new(),
            source_session_id: session_id.to_string(),
            source_turn_id: String::new(),
            source_text_hash: String::new(),
            source_kind: GoalSourceKind::UserProvided,
            confidence: 1.0,
        })
        .await?;
    Ok(format!("Goal set: {objective}"))
}
