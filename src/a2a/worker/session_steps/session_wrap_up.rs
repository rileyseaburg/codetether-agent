//! Append the shared budget wrap-up message to the application session.
use crate::session::Session;
pub(super) fn maybe_inject(session: &mut Session, step: usize, max_steps: usize) -> bool {
    if let Some(message) =
        codetether_a2a_worker_core::session_protocol::wrap_up_message(step, max_steps)
    {
        session.add_message(message);
        true
    } else {
        false
    }
}
