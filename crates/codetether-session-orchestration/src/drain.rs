//! Broadcast each newly accepted steering message to delegated child cursors.
use crate::Session;
use codetether_session_control::ControlHandle;
use codetether_session_steering as steering;

/// Drain accepted input and broadcast the newly appended messages to children.
/// # Arguments
/// `session` receives user input; `control` owns child input cursors;
/// `closing` selects atomic inbox closure when no pending input remains.
/// # Returns
/// The number of inputs appended to the transcript.
/// # Panics
/// Panics if the control input-log mutex is poisoned, or if a transcript adapter
/// removes existing messages while inserting steering input.
/// # Examples
/// ```rust,no_run
/// use codetether_session_orchestration::{Session, drain};
/// use codetether_session_control::ControlHandle;
/// # fn example<S: Session>(session: &mut S, control: &ControlHandle) {
/// let count = drain(session, control, false);
/// # }
/// ```
pub fn drain<S: Session>(session: &mut S, control: &ControlHandle, closing: bool) -> usize {
    let before = session.messages().len();
    let count = if closing {
        steering::drain_or_close_into(session)
    } else {
        steering::drain_into(session)
    };
    for message in &session.messages()[before..] {
        control.publish_input(message.clone());
    }
    count
}
