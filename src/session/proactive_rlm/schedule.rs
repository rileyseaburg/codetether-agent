//! Bind shared scheduling to native session identity and worker execution.
use crate::session::Session;
pub(super) fn run(session: &Session) {
    codetether_proactive_runtime::schedule(
        &session.id,
        session.messages.len(),
        super::worker::spawn,
    );
}
