//! `codetether run` checkpoint/resume orchestration helpers.

use crate::RunResumePlan;
use anyhow::Result;
use codetether_run_checkpoint::auto_resume_prompt;
use std::path::{Path, PathBuf};

/// Require transcript progress and a positive budget before saving exhaustion.
/// # Arguments
/// * `message_count_before` - Transcript length before the attempt.
/// * `session` - Current transcript.
/// * `max_steps` - Effective step budget.
/// # Returns
/// True when the transcript grew and the budget is positive.
/// # Examples
/// ```no_run
/// # fn example<S: codetether_run_resume::RunSession>(session: &S) {
/// assert!(!codetether_run_resume::should_checkpoint(0, session, 0));
/// # }
/// ```
pub fn should_checkpoint<S: crate::RunSession>(
    message_count_before: usize,
    session: &S,
    max_steps: usize,
) -> bool {
    session.message_count() > message_count_before && max_steps > 0
}

/// Persist a checkpoint when the step budget is exhausted.
///
/// Extracts real browser URL, completed actions, blockers, and next intended
/// action from the session message history.
/// # Arguments
/// * `session` - Session providing identity, transcript and persistence.
/// * `objective` - Original user request.
/// * `max_steps` - Exhausted step budget.
/// * `workspace` - Workspace recorded in the checkpoint.
/// # Returns
/// The persisted checkpoint path.
/// # Errors
/// Returns sidecar or session persistence errors.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_run_resume::RunSession>(session: &mut S) -> anyhow::Result<()> {
/// codetether_run_resume::persist_exhaustion_checkpoint(session, "task", 50, std::path::Path::new(".")).await?;
/// # Ok(()) }
/// ```
pub async fn persist_exhaustion_checkpoint<S: crate::RunSession>(
    session: &mut S,
    objective: &str,
    max_steps: usize,
    workspace: &Path,
) -> Result<PathBuf> {
    let cp = session
        .build_checkpoint(objective, max_steps, workspace)
        .await?;
    codetether_run_checkpoint::storage::save(session, cp).await
}

/// Load a checkpoint and render the continuation prompt.
/// # Arguments
/// * `session` - Existing session checkpoint storage.
/// * `remaining` - Retry count to retain in the plan.
/// # Returns
/// A continuation plan, or none when no checkpoint exists.
/// # Errors
/// Returns checkpoint loading errors.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_run_resume::RunSession>(session: &S) -> anyhow::Result<()> {
/// let plan = codetether_run_resume::resume_plan(session, 1).await?;
/// # Ok(()) }
/// ```
pub async fn resume_plan<S: crate::RunSession>(
    session: &S,
    remaining: usize,
) -> Result<Option<RunResumePlan>> {
    let Some(checkpoint) = codetether_run_checkpoint::storage::load(session).await? else {
        return Ok(None);
    };
    anyhow::ensure!(
        checkpoint.session_id == session.id(),
        "checkpoint belongs to another session"
    );
    // Reconcile edits and newer corrections even when the message count is unchanged.
    let checkpoint = session
        .build_checkpoint(
            &checkpoint.original_objective,
            checkpoint.max_step_budget,
            checkpoint
                .workspace
                .as_deref()
                .unwrap_or_else(|| Path::new(".")),
        )
        .await?;
    Ok(Some(RunResumePlan {
        prompt: auto_resume_prompt(&checkpoint),
        remaining,
    }))
}
