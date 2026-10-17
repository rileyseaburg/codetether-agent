//! Goal gates shared by every execution loop and frontend.

use anyhow::{Result, bail};

/// Refuse execution while an authenticated user hold or budget gate is active.
///
/// Completed goals do not prevent new user work. Only active unfinished goals
/// may continue automatically; this check never changes durable goal state.
/// # Errors
/// Returns journal errors or an explicit hold diagnostic, never an empty state.
pub(crate) async fn ensure_running(session_id: &str) -> Result<()> {
    if super::CONTROL.is_cancelled(session_id) {
        bail!("Session execution cancelled");
    }
    let (_, state) = super::goals::current(session_id).await?;
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
