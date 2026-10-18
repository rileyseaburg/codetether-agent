//! User-message extraction for prior-context directives.

use super::directive::Directive;
use codetether_provider_core::{Message, Role};

/// Return whether an iterator item is a user message with textual content.
pub fn is_human_text(message: &&Message) -> bool {
    message.role == Role::User && !text(message).trim().is_empty()
}

/// Classify the last authoritative directive in one user message.
/// # Arguments
/// One message whose textual clauses should be interpreted.
/// # Returns
/// The last authoritative directive, ignoring quoted examples.
/// # Examples
/// ```
/// let message = codetether_provider_core::Message { role: codetether_provider_core::Role::User, content: vec![] };
/// assert!(codetether_prior_context_policy::classify(&message).is_none());
/// ```
pub fn classify(message: &Message) -> Option<Directive> {
    super::normalize::clauses(&text(message))
        .iter()
        .rev()
        .find_map(|clause| super::directive_clause::classify(clause))
}

fn text(message: &Message) -> String {
    codetether_context_policy::transcript::extract_text_content(&message.content)
}
