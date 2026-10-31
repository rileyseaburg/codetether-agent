//! Prior-context-gated automatic recall injection.
/// Prepare a local evidence message using the session's existing policy.
/// # Arguments
/// * `session` - Current transcript, workspace and authorization decision.
/// # Returns
/// Earlier evidence from this session, or None when unavailable or forbidden.
/// # Examples
/// `message(&session).await` checks prior-context access before querying.
pub async fn message<S: crate::Session>(session: &S) -> Option<codetether_provider_core::Message> {
    codetether_recall_context::message(session, &crate::Host::<S>::default()).await
}
