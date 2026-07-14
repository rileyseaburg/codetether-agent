//! User-message extraction for prior-context directives.

use super::directive::Directive;
use crate::provider::{Message, Role};

/// Return whether an iterator item is a user message with textual content.
pub(super) fn is_human_text(message: &&Message) -> bool {
    message.role == Role::User && !text(message).trim().is_empty()
}

/// Classify the last authoritative directive in one user message.
pub(super) fn classify(message: &Message) -> Option<Directive> {
    super::normalize::clauses(&text(message))
        .iter()
        .rev()
        .find_map(|clause| super::directive_clause::classify(clause))
}

fn text(message: &Message) -> String {
    crate::session::helper::text::extract_text_content(&message.content)
}
