//! Transfer queued steering inputs into the trusted transcript.

use crate::session::Session;

/// Append all currently queued inputs while leaving the run open.
pub(crate) fn drain_into(session: &mut Session) -> usize {
    append(session, super::queue::take(&session.id))
}

/// Atomically append pending inputs or close an empty run to new input.
pub(crate) fn drain_or_close_into(session: &mut Session) -> usize {
    append(session, super::queue::take_or_close(&session.id))
}

fn append(session: &mut Session, inputs: Vec<super::SteeringInput>) -> usize {
    let count = inputs.len();
    for input in inputs {
        crate::tool::agent::collaboration_runtime::parent_activity::acknowledge_steered(
            &session.id,
            input.activity_id(),
        );
        let (message, text) = input.into_message();
        session.add_human_message(message);
        super::super::publish_user_prompt::publish(session, &text, None);
    }
    if count > 0 {
        tracing::info!(session_id = %session.id, input_count = count, "Applied steering input");
    }
    count
}
