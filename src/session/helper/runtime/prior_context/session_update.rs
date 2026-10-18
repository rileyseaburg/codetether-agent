//! Native session binding for trusted and delegated policy transitions.
use crate::{provider::Message, session::Session};
pub(super) fn trusted(session: &mut Session, message: &Message) {
    codetether_prior_context_access::trusted(&mut session.metadata, &session.messages, message);
}
pub(super) fn delegated(session: &mut Session, message: &Message) {
    codetether_prior_context_access::delegated(&mut session.metadata, &session.messages, message);
}
