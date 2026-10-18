//! Application-local session view and authoritative message-ingress binding.
mod ingress;
pub use codetether_provider_core::Message;
pub use codetether_session_metadata::SessionMetadata;

/// Borrow the existing prior-context metadata and transcript.
/// # Implementors
/// Generated for the local session by [crate::impl_session_ingress!].
pub trait Source {
    /// Existing policy metadata.
    fn metadata(&self) -> &SessionMetadata;
    /// Existing transcript used for fallback policy inference.
    fn messages(&self) -> &[Message];
}
/// Resolve access using the session's existing metadata and transcript.
/// # Arguments
/// * `session` - Borrowed application session view.
/// # Returns
/// The shared prior-context permission decision.
/// # Examples
/// ```no_run
/// # fn example(session: &impl codetether_prior_context_access::session_binding::Source) {
/// let _allowed = codetether_prior_context_access::session_binding::allowed_for_session(session);
/// # }
/// ```
pub fn allowed_for_session(session: &impl Source) -> bool {
    crate::allowed(session.metadata(), session.messages())
}
