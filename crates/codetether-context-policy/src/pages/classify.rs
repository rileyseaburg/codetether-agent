//! Syntactic page classification for provider messages.
use super::*;

/// Classify a single chat-history entry.
///
/// Heuristic (Phase A):
///
/// * [`Role::System`] or [`Role::Developer`] → [`PageKind::Bootstrap`].
/// * [`Role::Tool`] or any content part that is
///   [`ContentPart::ToolResult`] → [`PageKind::Evidence`].
/// * Everything else → [`PageKind::Conversation`].
///
/// [`PageKind::Constraint`], [`PageKind::Plan`], and
/// [`PageKind::Preference`] require explicit producers (the
/// `.tasks.jsonl` loader, swarm objectives, user prefs block) and are
/// not reachable from this syntactic classifier. That is intentional —
/// they are load-bearing invariant tags, not best-effort guesses.
///
/// # Examples
///
/// ```rust
/// use codetether_provider_core::{ContentPart, Message, Role};
/// use codetether_context_policy::pages::{PageKind, classify};
///
/// let user = Message {
///     role: Role::User,
///     content: vec![ContentPart::Text { text: "hi".into() }],
/// };
/// assert_eq!(classify(&user), PageKind::Conversation);
/// ```
pub fn classify(msg: &Message) -> PageKind {
    if matches!(msg.role, Role::System | Role::Developer) {
        return PageKind::Bootstrap;
    }
    if matches!(msg.role, Role::Tool) {
        return PageKind::Evidence;
    }
    for part in &msg.content {
        if matches!(part, ContentPart::ToolResult { .. }) {
            return PageKind::Evidence;
        }
    }
    PageKind::Conversation
}

/// Classify every entry in `messages` and return the parallel array.
///
/// Used at session load to backfill the `pages` sidecar for legacy
/// sessions that predate the Phase A refactor.
pub fn classify_all(messages: &[Message]) -> Vec<PageKind> {
    messages.iter().map(classify).collect()
}
