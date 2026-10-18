//! Route final messages to accepted steering or existing goal continuation.
use crate::{Continuation, Session};
use anyhow::Result;
use codetether_goal_runtime::Journal;
use codetether_session_steering as steering;

/// Drain accepted input or schedule the existing goal after a final message.
///
/// # Errors
/// Returns journal errors instead of treating failed continuation as completion.
/// # Arguments
/// `session` is the live transcript; `J` resolves its existing goal journal.
/// # Returns
/// A continuation reason without marking the goal complete or changing its objective.
/// # Examples
/// ```rust,no_run
/// use codetether_session_orchestration::{Session, after_message};
/// use codetether_goal_runtime::Journal;
/// # async fn example<J: Journal, S: Session>(session: &mut S) -> anyhow::Result<()> {
/// let reason = after_message::<J, S>(session).await?;
/// # Ok(()) }
/// ```
pub async fn after_message<J: Journal, S: Session>(session: &mut S) -> Result<Continuation> {
    if steering::drain_into(session) > 0 {
        return Ok(Continuation::Steering);
    }
    let next = continue_goal::<J, S>(session).await?;
    if matches!(next, Continuation::Finish) && steering::drain_or_close_into(session) > 0 {
        return Ok(Continuation::Steering);
    }
    Ok(next)
}

/// Schedule only an active goal, preserving its objective and accounting.
///
/// # Errors
/// Returns journal errors without silently ending the task.
/// # Arguments
/// `session` is the live transcript; `J` resolves its existing goal journal.
/// # Returns
/// A continuation reason without marking the goal complete or changing its objective.
/// # Examples
/// ```rust,no_run
/// use codetether_session_orchestration::{Session, continue_goal};
/// use codetether_goal_runtime::Journal;
/// # async fn example<J: Journal, S: Session>(session: &mut S) -> anyhow::Result<()> {
/// let reason = continue_goal::<J, S>(session).await?;
/// # Ok(()) }
/// ```
pub async fn continue_goal<J: Journal, S: Session>(session: &mut S) -> Result<Continuation> {
    match codetether_goal_runtime::next_message::<J>(session.id()).await? {
        Some(message) => {
            session.add_message(message);
            steering::open(session.id());
            Ok(Continuation::Goal)
        }
        None => Ok(Continuation::Finish),
    }
}
