//! Session persistence adapter for the shared pinned-state header.
use crate::provider::Message;
use crate::session::Session;
pub(super) fn prepend_state_header(session: &Session, messages: &mut Vec<Message>) {
    codetether_context_policy::state_header::prepend_state_header(
        crate::session::store::constraints::prefix(session),
        &session.pages,
        &session.messages,
        || session.message_offset(),
        messages,
    );
}
