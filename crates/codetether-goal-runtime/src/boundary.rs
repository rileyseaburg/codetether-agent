//! Goal gates shared by every execution loop and frontend.

use crate::Journal;
use anyhow::{Result, bail};

/// Refuse execution while an authenticated user hold or budget gate is active.
///
/// Completed goals do not prevent new user work. Only active unfinished goals
/// may continue automatically; this check never changes durable goal state.
/// # Errors
/// Returns journal errors or an explicit hold diagnostic, never an empty state.
/// Check cancellation and durable user holds before executing another step.
/// # Arguments
/// `session_id` selects the shared control state and goal journal.
/// # Returns
/// Returns unit when execution may proceed.
/// # Errors
/// Execution is cancelled, answer review or a nonterminal hold is active, or journal loading fails.
/// # Examples
/// ```rust,no_run
/// use codetether_goal_runtime::{Journal, ensure_running};
/// # async fn example<H: Journal>() -> anyhow::Result<()> {
/// ensure_running::<H>("session-1234").await?;
/// # Ok(()) }
/// ```
pub async fn ensure_running<H: Journal>(session_id: &str) -> Result<()> {
    if codetether_session_control::CONTROL.is_cancelled(session_id) {
        bail!("Session execution cancelled");
    }
    let (_, state) = crate::current::<H>(session_id).await?;
    if state.answer_review.is_some() {
        bail!("Goal paused for answer review; submit /continue to resume");
    }
    if let Some(goal) = state.goal
        && !goal.status.is_active()
        && !goal.status.is_terminal()
    {
        bail!("Goal execution is {}", goal.status.as_str());
    }
    Ok(())
}
