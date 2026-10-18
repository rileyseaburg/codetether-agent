//! Manual continuation prompt for a persisted active goal.

use super::prompt::render;
use crate::Journal;
use crate::state_cache;

/// Render manual continuation text if an active goal can be read.
/// # Arguments
/// `session_id` selects the journal.
/// # Returns
/// Returns text for an active goal, or `None` on absent/inactive/unreadable state.
/// # Examples
/// ```rust,no_run
/// use codetether_goal_runtime::{Journal, resume_prompt};
/// # fn example<H: Journal>() {
/// let value = resume_prompt::<H>("session-1234");
/// # }
/// ```
pub fn prompt<H: Journal>(session_id: &str) -> Option<String> {
    let log = H::for_session(session_id).ok()?;
    state_cache::load(&log)
        .ok()?
        .goal
        .filter(|goal| goal.status.is_active())
        .map(|goal| render(&goal))
}
