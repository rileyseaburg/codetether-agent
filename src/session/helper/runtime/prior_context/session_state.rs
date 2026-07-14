//! Durable session state for prior-context access policy.

use crate::session::Session;

/// Resolve effective access using messages plus the durable policy baseline.
pub(crate) fn allowed(session: &Session) -> bool {
    if session.metadata.inherited_prior_context_allowed == Some(false) {
        return false;
    }
    session
        .metadata
        .prior_context_turn_allowed
        .or(session.metadata.prior_context_allowed)
        .unwrap_or_else(|| super::directive::resolve(&session.messages))
}
