//! Goal reaffirmation and clearing persistence.
use crate::Host;

use anyhow::{Result, anyhow};
use chrono::Utc;
use codetether_goal_core::{GoalReaffirmation, TaskEvent};

pub(super) async fn reaffirm<H: Host>(session_id: &str, note: &str) -> Result<String> {
    if note.is_empty() {
        return Err(anyhow!("usage: /goal reaffirm <progress note>"));
    }
    H::log(session_id)?
        .append(&TaskEvent::GoalReaffirmed(GoalReaffirmation {
            at: Utc::now(),
            progress_note: note.into(),
        }))
        .await?;
    Ok(format!("Goal reaffirmed: {note}"))
}

pub(super) async fn clear<H: Host>(session_id: &str, reason: &str) -> Result<String> {
    let reason = if reason.is_empty() {
        "cleared by user"
    } else {
        reason
    };
    H::log(session_id)?
        .append(&TaskEvent::GoalCleared {
            at: Utc::now(),
            reason: reason.into(),
        })
        .await?;
    Ok(format!("Goal cleared: {reason}"))
}
