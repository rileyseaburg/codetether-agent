//! Current-state loading for goal runtime operations.

use crate::Journal;
use crate::state_cache;
use anyhow::Result;
use codetether_goal_core::{TaskLog, TaskState};

/// Load the session task log and its folded state.
///
/// Uses the length/mtime-keyed fold cache so per-step usage accounting does
/// not re-parse an ever-growing log on every model step.
/// Resolve the journal and load its cached folded state.
/// # Arguments
/// `session_id` selects the journal through the application resolver.
/// # Returns
/// Returns the journal handle and its current folded state.
/// # Errors
/// Journal resolution or reading an existing journal fails.
/// # Examples
/// ```rust,no_run
/// use codetether_goal_runtime::{Journal, current};
/// # async fn example<H: Journal>() -> anyhow::Result<()> {
/// let (journal, state) = current::<H>("session-1234").await?;
/// # Ok(()) }
/// ```
pub async fn current<H: Journal>(session_id: &str) -> Result<(TaskLog, TaskState)> {
    let log = H::for_session(session_id)?;
    let state = state_cache::load(&log)?;
    Ok((log, state))
}
