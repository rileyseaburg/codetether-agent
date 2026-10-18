//! Native session view for shared prior-context permission precedence.
use crate::session::Session;
pub(crate) fn allowed(session: &Session) -> bool {
    codetether_prior_context_access::allowed(&session.metadata, &session.messages)
}
