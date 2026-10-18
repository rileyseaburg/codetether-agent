//! Trusted and delegated updates to durable prior-context policy.

use codetether_provider_core::Message;
use codetether_session_metadata::SessionMetadata;

use codetether_prior_context_policy::Directive;

/// # Arguments
/// Receives mutable metadata, the existing transcript and an incoming message.
/// # Returns
/// Updates the supplied state in place.
/// # Examples
/// ```
/// let mut metadata = codetether_session_metadata::SessionMetadata::default();
/// let message = codetether_provider_core::Message { role: codetether_provider_core::Role::User, content: vec![] };
/// codetether_prior_context_access::trusted(&mut metadata, &[], &message);
/// assert_eq!(metadata.prior_context_allowed, Some(true));
/// ```
pub fn trusted(metadata: &mut SessionMetadata, messages: &[Message], message: &Message) {
    if !super::prepare::prepare(metadata, messages, message) {
        return;
    }
    metadata.prior_context_turn_allowed = None;
    match codetether_prior_context_policy::classify(message) {
        Some(Directive::AllowOnce) => metadata.prior_context_turn_allowed = Some(true),
        Some(Directive::DenyOnce) => metadata.prior_context_turn_allowed = Some(false),
        Some(Directive::AllowPersistent) => metadata.prior_context_allowed = Some(true),
        Some(Directive::DenyPersistent) => metadata.prior_context_allowed = Some(false),
        None => {}
    }
}

/// # Arguments
/// Receives mutable metadata, the existing transcript and an incoming message.
/// # Returns
/// Updates the supplied state in place.
/// # Examples
/// ```
/// let mut metadata = codetether_session_metadata::SessionMetadata::default();
/// let message = codetether_provider_core::Message { role: codetether_provider_core::Role::User, content: vec![] };
/// codetether_prior_context_access::delegated(&mut metadata, &[], &message);
/// assert_eq!(metadata.prior_context_allowed, Some(true));
/// ```
pub fn delegated(metadata: &mut SessionMetadata, messages: &[Message], message: &Message) {
    if !super::prepare::prepare(metadata, messages, message) {
        return;
    }
    match codetether_prior_context_policy::classify(message) {
        Some(Directive::DenyOnce) => metadata.prior_context_turn_allowed = Some(false),
        Some(Directive::DenyPersistent) => metadata.prior_context_allowed = Some(false),
        Some(Directive::AllowOnce | Directive::AllowPersistent) | None => {}
    }
}
