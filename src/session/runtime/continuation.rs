//! Shared final-message routing; message completion is not goal completion.

use crate::session::Session;
use anyhow::Result;

/// Reason an execution loop should run another model step.
pub(crate) enum Continuation {
    /// A human correction arrived before the inbox closed.
    Steering,
    /// The existing durable goal is still active.
    Goal,
    /// No accepted steering or active goal requires further execution.
    Finish,
}

/// Drain accepted input or schedule the existing goal after a final message.
///
/// # Errors
/// Returns journal errors instead of treating failed continuation as completion.
pub(crate) async fn after_message(session: &mut Session) -> Result<Continuation> {
    if super::steering::drain_into(session) > 0 {
        return Ok(Continuation::Steering);
    }
    let next = continue_goal(session).await?;
    if matches!(next, Continuation::Finish) && super::steering::drain_or_close_into(session) > 0 {
        return Ok(Continuation::Steering);
    }
    Ok(next)
}

/// Schedule only an active goal, preserving its objective and accounting.
///
/// # Errors
/// Returns journal errors without silently ending the task.
pub(crate) async fn continue_goal(session: &mut Session) -> Result<Continuation> {
    match super::goals::next_message(&session.id).await? {
        Some(message) => {
            session.add_message(message);
            super::steering::open(&session.id);
            Ok(Continuation::Goal)
        }
        None => Ok(Continuation::Finish),
    }
}
