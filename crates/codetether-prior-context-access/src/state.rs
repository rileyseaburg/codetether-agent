//! Durable session state for prior-context access policy.

use codetether_provider_core::Message;
use codetether_session_metadata::SessionMetadata;

/// Resolve effective access using messages plus the durable policy baseline.
/// # Arguments
/// Receives durable metadata and the current transcript.
/// # Returns
/// The effective permission or detection decision.
/// # Examples
/// ```
/// let metadata = codetether_session_metadata::SessionMetadata::default();
/// assert!(codetether_prior_context_access::allowed(&metadata, &[]));
/// ```
pub fn allowed(metadata: &SessionMetadata, messages: &[Message]) -> bool {
    if metadata.inherited_prior_context_allowed == Some(false) {
        return false;
    }
    metadata
        .prior_context_turn_allowed
        .or(metadata.prior_context_allowed)
        .unwrap_or_else(|| codetether_prior_context_policy::resolve(messages))
}
