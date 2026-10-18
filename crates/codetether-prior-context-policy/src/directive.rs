//! Resolution of persistent and one-turn prior-context directives.

use codetether_provider_core::Message;

/// Access-policy directive extracted from one user clause.
/// # Examples
/// ```
/// use codetether_prior_context_policy::Directive;
/// let allowed = match Directive::AllowOnce {
///     Directive::AllowOnce | Directive::AllowPersistent => true,
///     Directive::DenyOnce | Directive::DenyPersistent => false,
/// };
/// assert!(allowed);
/// ```
#[derive(Clone, Copy)]
pub enum Directive {
    /// Allow prior context only for the current turn.
    AllowOnce,
    /// Allow prior context on subsequent turns.
    AllowPersistent,
    /// Deny prior context only for the current turn.
    DenyOnce,
    /// Deny prior context until an authoritative permission restores it.
    DenyPersistent,
}

/// Resolve policy from messages with access allowed as the legacy default.
/// # Arguments
/// Ordered transcript messages.
/// # Returns
/// Effective permission, with legacy access enabled when no directive exists.
/// # Examples
/// ```
/// assert!(codetether_prior_context_policy::resolve(&[]));
/// ```
pub fn resolve(messages: &[Message]) -> bool {
    resolve_with_default(messages, true)
}

/// Resolve policy using `default` when no persistent directive is available.
/// # Arguments
/// Ordered transcript messages and fallback permission.
/// # Returns
/// Effective permission after temporary and persistent directives.
/// # Examples
/// ```
/// assert!(!codetether_prior_context_policy::resolve_with_default(&[], false));
/// ```
pub fn resolve_with_default(messages: &[Message], default: bool) -> bool {
    let mut users = messages
        .iter()
        .rev()
        .filter(|message| super::directive_message::is_human_text(message));
    if let Some(latest) = users.next().and_then(super::directive_message::classify) {
        return matches!(latest, Directive::AllowOnce | Directive::AllowPersistent);
    }
    users
        .filter_map(super::directive_message::classify)
        .find_map(|directive| match directive {
            Directive::AllowPersistent => Some(true),
            Directive::DenyPersistent => Some(false),
            Directive::AllowOnce | Directive::DenyOnce => None,
        })
        .unwrap_or(default)
}

/// Extract a persistent policy update from one user message.
/// # Arguments
/// One provider message to classify.
/// # Returns
/// A persistent permission update, or None for temporary/absent directives.
/// # Examples
/// ```
/// let message = codetether_provider_core::Message { role: codetether_provider_core::Role::User, content: vec![] };
/// assert_eq!(codetether_prior_context_policy::persistent_update(&message), None);
/// ```
pub fn persistent_update(message: &Message) -> Option<bool> {
    match super::directive_message::classify(message)? {
        Directive::AllowPersistent => Some(true),
        Directive::DenyPersistent => Some(false),
        Directive::AllowOnce | Directive::DenyOnce => None,
    }
}

/// Find the newest persistent directive in a transcript.
/// # Arguments
/// Ordered transcript messages.
/// # Returns
/// The newest persistent directive, or None when absent.
/// # Examples
/// ```
/// assert_eq!(codetether_prior_context_policy::latest_persistent(&[]), None);
/// ```
pub fn latest_persistent(messages: &[Message]) -> Option<bool> {
    messages
        .iter()
        .rev()
        .filter(super::directive_message::is_human_text)
        .find_map(persistent_update)
}
