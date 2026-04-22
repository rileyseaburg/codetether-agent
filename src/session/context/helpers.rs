//! Shared types and tiny helpers used across the context derivation modules.

use crate::provider::Message;
use crate::session::ResidencyLevel;

/// The per-step LLM context, derived from an append-only chat history.
///
/// This is the object the prompt loop should hand to the provider — *not*
/// [`Session::messages`](crate::session::Session::messages) directly.
///
/// # Fields
///
/// * `messages` — The message list to include in the completion request.
/// * `origin_len` — Length of the source history at the moment of derivation.
/// * `compressed` — Whether compression fired during this derivation.
/// * `dropped_ranges` — Best-effort ranges elided from the working context.
/// * `provenance` — Pipeline steps that produced this context.
/// * `resolutions` — Per-message residency levels in the derived context.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::session::context::DerivedContext;
///
/// let derived = DerivedContext {
///     messages: Vec::new(),
///     origin_len: 0,
///     compressed: false,
///     dropped_ranges: Vec::new(),
///     provenance: Vec::new(),
///     resolutions: Vec::new(),
/// };
/// assert_eq!(derived.origin_len, 0);
/// assert!(!derived.compressed);
/// ```
#[derive(Debug, Clone)]
pub struct DerivedContext {
    /// Messages to send to the provider this turn.
    pub messages: Vec<Message>,
    /// `session.messages.len()` at the moment of derivation.
    pub origin_len: usize,
    /// `true` when any compression / truncation pass rewrote the clone.
    pub compressed: bool,
    /// Best-effort elided ranges from the source history.
    pub dropped_ranges: Vec<(usize, usize)>,
    /// Names of the pipeline stages that shaped this context.
    pub provenance: Vec<String>,
    /// Per-message residency level in the derived context.
    pub resolutions: Vec<ResidencyLevel>,
}

/// Compare two message counts and return whether compression fired.
///
/// Separated out so the comparison is testable without a full provider
/// round-trip. Any count change is treated as evidence that compression
/// fired.
pub(super) fn messages_len_changed(before: usize, after: &[Message]) -> bool {
    before != after.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::{ContentPart, Role};

    #[test]
    fn derived_context_record_round_trips() {
        let ctx = DerivedContext {
            messages: vec![Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: "hi".to_string(),
                }],
            }],
            origin_len: 1,
            compressed: false,
            dropped_ranges: Vec::new(),
            provenance: Vec::new(),
            resolutions: vec![ResidencyLevel::Full],
        };
        let cloned = ctx.clone();
        assert_eq!(ctx.origin_len, cloned.origin_len);
        assert_eq!(ctx.compressed, cloned.compressed);
        assert_eq!(ctx.messages.len(), cloned.messages.len());
        assert_eq!(ctx.resolutions, cloned.resolutions);
    }

    #[test]
    fn messages_len_changed_detects_shrink_and_noop() {
        let empty: Vec<Message> = Vec::new();
        assert!(!messages_len_changed(0, &empty));

        let one = vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "x".to_string(),
            }],
        }];
        assert!(messages_len_changed(5, &one));
        assert!(!messages_len_changed(1, &one));
    }
}
