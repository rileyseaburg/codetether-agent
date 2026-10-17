//! Prepend the formatted state header after context derivation.
use super::*;

/// Prepend a compact `[CONTEXT STATE]` message containing hard-pinned
/// constraint turns. This is injected *after* derivation, so pins survive
/// Legacy compression, Reset summaries, and Incremental selection.
///
/// # Arguments
/// * `prefix` - Persisted constraint lines, already rendered by the host.
/// * `pages` - Page classifications for the live window.
/// * `source` - Live messages corresponding to those classifications.
/// * `offset` - Current transcript offset, read lazily for each included live pin.
/// * `messages` - Derived message buffer receiving the header.
///
/// # Returns
/// Nothing; a header is inserted only when constraints exist.
///
/// # Examples
/// ```
/// use codetether_context_policy::state_header::prepend_state_header;
/// let mut messages = Vec::new();
/// prepend_state_header(vec!["- turn 0: retain evidence".into()], &[], &[], || 0, &mut messages);
/// assert_eq!(messages.len(), 1);
/// ```
pub fn prepend_state_header(
    prefix: Vec<String>,
    pages: &[PageKind],
    source: &[Message],
    offset: impl Fn() -> usize,
    messages: &mut Vec<Message>,
) {
    let constraints = pinned_constraints(prefix, pages, source, &offset);
    if constraints.is_empty() {
        return;
    }
    let header = Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text {
            text: format!(
                "[CONTEXT STATE]\nHard constraints pinned by the user; obey even if older context was compressed:\n{}",
                constraints.join("\n")
            ),
        }],
    };
    messages.insert(0, header);
}
