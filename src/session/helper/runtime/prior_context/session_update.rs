//! Trusted and delegated updates to durable prior-context policy.

use crate::provider::{Message, Role};
use crate::session::Session;

use super::directive::Directive;

pub(super) fn trusted(session: &mut Session, message: &Message) {
    if !prepare(session, message) {
        return;
    }
    session.metadata.prior_context_turn_allowed = None;
    match super::directive_message::classify(message) {
        Some(Directive::AllowOnce) => session.metadata.prior_context_turn_allowed = Some(true),
        Some(Directive::DenyOnce) => session.metadata.prior_context_turn_allowed = Some(false),
        Some(Directive::AllowPersistent) => session.metadata.prior_context_allowed = Some(true),
        Some(Directive::DenyPersistent) => session.metadata.prior_context_allowed = Some(false),
        None => {}
    }
}

pub(super) fn delegated(session: &mut Session, message: &Message) {
    if !prepare(session, message) {
        return;
    }
    match super::directive_message::classify(message) {
        Some(Directive::DenyOnce) => session.metadata.prior_context_turn_allowed = Some(false),
        Some(Directive::DenyPersistent) => session.metadata.prior_context_allowed = Some(false),
        Some(Directive::AllowOnce | Directive::AllowPersistent) | None => {}
    }
}

fn prepare(session: &mut Session, message: &Message) -> bool {
    if message.role != Role::User {
        return false;
    }
    if session.metadata.prior_context_allowed.is_none() {
        session.metadata.prior_context_allowed =
            Some(super::directive::latest_persistent(&session.messages).unwrap_or(true));
    }
    true
}
