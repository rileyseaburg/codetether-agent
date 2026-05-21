//! `codetether run` checkpoint/resume orchestration helpers.

use crate::session::{RunCheckpoint, Session, auto_resume_prompt};
use anyhow::Result;
use std::path::{Path, PathBuf};

/// A continuation plan assembled from a saved checkpoint.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::cli::run_checkpoint::RunResumePlan;
///
/// let plan = RunResumePlan {
///     prompt: "Continue the task.".to_string(),
///     remaining: 3,
/// };
/// assert_eq!(plan.remaining, 3);
/// ```
pub struct RunResumePlan {
    /// The continuation prompt assembled from the checkpoint data.
    pub prompt: String,
    /// How many auto-continue attempts remain.
    pub remaining: usize,
}

/// Validate that `--auto-continue-until` is at least 1 when provided.
///
/// # Errors
///
/// Returns `Err` if `limit` is `Some(0)`.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::cli::run_checkpoint::validate_auto_continue;
///
/// assert!(validate_auto_continue(Some(0)).is_err());
/// assert!(validate_auto_continue(Some(1)).is_ok());
/// assert!(validate_auto_continue(None).is_ok());
/// ```
pub fn validate_auto_continue(limit: Option<usize>) -> Result<()> {
    if matches!(limit, Some(0)) {
        anyhow::bail!("--auto-continue-until must be at least 1");
    }
    Ok(())
}

/// Return `true` when the session accumulated new messages and step budget > 0.
pub fn should_checkpoint(message_count_before: usize, session: &Session, max_steps: usize) -> bool {
    session.messages.len() > message_count_before && max_steps > 0
}

/// Persist an exhaustion checkpoint for the current session.
pub async fn persist_exhaustion_checkpoint(
    session: &mut Session,
    objective: &str,
    max_steps: usize,
    workspace: &Path,
) -> Result<PathBuf> {
    let cp = RunCheckpoint::exhausted(
        objective,
        max_steps,
        session.id.clone(),
        Some(workspace.to_path_buf()),
        session.messages.len(),
    );
    session.save_run_checkpoint(cp).await
}

/// Build a [`RunResumePlan`] from the session's saved checkpoint, if any.
pub async fn resume_plan(session: &Session, remaining: usize) -> Result<Option<RunResumePlan>> {
    let Some(checkpoint) = session.load_run_checkpoint().await? else {
        return Ok(None);
    };
    Ok(Some(RunResumePlan {
        prompt: auto_resume_prompt(&checkpoint),
        remaining,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_zero_auto_continue_limit() {
        assert!(validate_auto_continue(Some(0)).is_err());
        assert!(validate_auto_continue(Some(1)).is_ok());
        assert!(validate_auto_continue(None).is_ok());
    }
}
