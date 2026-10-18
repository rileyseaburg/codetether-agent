//! Read the durable answer hold, failing closed on unreadable task logs.

use crate::Journal;
use crate::state_cache;
use anyhow::Result;
use codetether_goal_core::TaskState;

/// Load current goal and answer-review state.
/// # Arguments
/// `session` selects the journal.
/// # Returns
/// Returns the folded state.
/// # Errors
/// Journal resolution or reading an existing journal fails.
/// # Examples
/// ```rust,no_run
/// use codetether_goal_runtime::{Journal, answer_review};
/// # async fn example<H: Journal>() -> anyhow::Result<()> {
/// let state = answer_review::read::<H>("session-1234")?;
/// # Ok(()) }
/// ```
pub fn read<H: Journal>(session: &str) -> Result<TaskState> {
    state_cache::load(&H::for_session(session)?)
}

/// Check whether answer review holds the session, failing closed on errors.
/// # Arguments
/// `session` selects the journal.
/// # Returns
/// Returns true for a recorded hold or a journal error.
/// # Examples
/// ```rust,no_run
/// use codetether_goal_runtime::{Journal, answer_review};
/// # fn example<H: Journal>() {
/// let value = answer_review::held::<H>("session-1234");
/// # }
/// ```
pub fn held<H: Journal>(session: &str) -> bool {
    match read::<H>(session) {
        Ok(state) => state.answer_review.is_some(),
        Err(error) => {
            tracing::warn!(target: "codetether_agent::session::runtime::goals::answer_review::query", %error, session, "Answer-review log unreadable; refusing continuation");
            true
        }
    }
}

/// Check whether the current answer-review response is ready.
/// # Arguments
/// `session` selects the journal.
/// # Returns
/// Returns true only for a readable review marked ready.
/// # Examples
/// ```rust,no_run
/// use codetether_goal_runtime::{Journal, answer_review};
/// # fn example<H: Journal>() {
/// let value = answer_review::ready::<H>("session-1234");
/// # }
/// ```
pub fn ready<H: Journal>(session: &str) -> bool {
    read::<H>(session)
        .ok()
        .and_then(|state| state.answer_review)
        .is_some_and(|review| review.ready)
}
