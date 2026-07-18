//! Goal reaffirmation and clearing persistence.

use crate::session::tasks::{TaskEvent, TaskLog};
use anyhow::{Result, anyhow};
use chrono::Utc;

pub(super) async fn reaffirm(session_id: &str, note: &str) -> Result<String> {
    if note.is_empty() {
        return Err(anyhow!("usage: /goal reaffirm <progress note>"));
    }
    TaskLog::for_session(session_id)?
        .append(&TaskEvent::GoalReaffirmed {
            at: Utc::now(),
            progress_note: note.into(),
        })
        .await?;
    Ok(format!("Goal reaffirmed: {note}"))
}

pub(super) async fn clear(session_id: &str, reason: &str) -> Result<String> {
    let reason = if reason.is_empty() {
        "cleared by user"
    } else {
        reason
    };
    TaskLog::for_session(session_id)?
        .append(&TaskEvent::GoalCleared {
            at: Utc::now(),
            reason: reason.into(),
        })
        .await?;
    Ok(format!("Goal cleared: {reason}"))
}
