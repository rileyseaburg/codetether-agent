//! Transfer queued steering inputs into the trusted transcript.

use crate::Transcript;

/// Append all currently queued inputs while leaving the run open.
/// # Arguments
/// A mutable native transcript implementing [Transcript].
/// # Returns
/// The number of inputs appended; the inbox remains open.
/// # Examples
/// ```rust
/// use codetether_session_steering::{Transcript, drain_into};
/// # fn example<S: Transcript>(session: &mut S) {
/// let count = drain_into(session);
/// # }
/// ```
pub fn drain_into<S: Transcript>(session: &mut S) -> usize {
    let inputs = super::queue::take(session.id());
    append(session, inputs)
}

/// Atomically append pending inputs or close an empty run to new input.
/// # Arguments
/// A mutable native transcript implementing [Transcript].
/// # Returns
/// The number appended; an empty inbox is closed atomically.
/// # Examples
/// ```rust
/// use codetether_session_steering::{Transcript, drain_or_close_into};
/// # fn example<S: Transcript>(session: &mut S) {
/// let count = drain_or_close_into(session);
/// # }
/// ```
pub fn drain_or_close_into<S: Transcript>(session: &mut S) -> usize {
    let inputs = super::queue::take_or_close(session.id());
    append(session, inputs)
}

fn append<S: Transcript>(session: &mut S, inputs: Vec<super::SteeringInput>) -> usize {
    let count = inputs.len();
    for input in inputs {
        codetether_collaboration_state::parent_activity::acknowledge_steered(
            session.id(),
            input.activity_id(),
        );
        let (message, text) = input.into_message();
        session.add_human_message(message);
        session.publish_user_prompt(&text);
    }
    if count > 0 {
        tracing::info!(target: "codetether_agent::session::runtime::steering::drain", session_id = %session.id(), input_count = count, "Applied steering input");
    }
    count
}
