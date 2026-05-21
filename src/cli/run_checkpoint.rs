//! `codetether run` checkpoint/resume orchestration helpers.

use crate::session::{RunCheckpoint, Session, auto_resume_prompt};
use anyhow::Result;
use std::path::{Path, PathBuf};

pub struct RunResumePlan {
    pub prompt: String,
    pub remaining: usize,
}

pub fn validate_auto_continue(limit: Option<usize>) -> Result<()> {
    if matches!(limit, Some(0)) {
        anyhow::bail!("--auto-continue-until must be at least 1");
    }
    Ok(())
}

pub fn should_checkpoint(message_count_before: usize, session: &Session, max_steps: usize) -> bool {
    session.messages.len() > message_count_before && max_steps > 0
}

/// Persist a checkpoint when the step budget is exhausted.
///
/// Extracts real browser URL, completed actions, blockers, and next intended
/// action from the session message history.
pub async fn persist_exhaustion_checkpoint(
    session: &mut Session,
    objective: &str,
    max_steps: usize,
    workspace: &Path,
) -> Result<PathBuf> {
    let cp = RunCheckpoint::from_session_messages(
        objective,
        max_steps,
        session.id.clone(),
        Some(workspace.to_path_buf()),
        session.messages.len(),
        &session.messages,
    );
    session.save_run_checkpoint(cp).await
}

pub async fn resume_plan(session: &Session, remaining: usize) -> Result<Option<RunResumePlan>> {
    let Some(checkpoint) = session.load_run_checkpoint().await? else {
        return Ok(None);
    };
    Ok(Some(RunResumePlan {
        prompt: auto_resume_prompt(&checkpoint),
        remaining,
    }))
}
