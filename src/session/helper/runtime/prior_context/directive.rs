//! Resolution of persistent and one-turn prior-context directives.

use crate::provider::Message;

/// Access-policy directive extracted from one user clause.
#[derive(Clone, Copy)]
pub(super) enum Directive {
    AllowOnce,
    AllowPersistent,
    DenyOnce,
    DenyPersistent,
}

/// Resolve policy from messages with access allowed as the legacy default.
pub(super) fn resolve(messages: &[Message]) -> bool {
    resolve_with_default(messages, true)
}

/// Resolve policy using `default` when no persistent directive is available.
pub(super) fn resolve_with_default(messages: &[Message], default: bool) -> bool {
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
pub(super) fn persistent_update(message: &Message) -> Option<bool> {
    match super::directive_message::classify(message)? {
        Directive::AllowPersistent => Some(true),
        Directive::DenyPersistent => Some(false),
        Directive::AllowOnce | Directive::DenyOnce => None,
    }
}

/// Find the newest persistent directive in a transcript.
pub(super) fn latest_persistent(messages: &[Message]) -> Option<bool> {
    messages
        .iter()
        .rev()
        .filter(super::directive_message::is_human_text)
        .find_map(persistent_update)
}
