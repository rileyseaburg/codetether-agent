//! Native specialization of shared state-header construction.
use crate::{provider::Message, session::Session};
pub(super) fn prepend_state_header(session: &Session, messages: &mut Vec<Message>) {
    codetether_context_dispatch::prepend_state_header(session, messages);
}
