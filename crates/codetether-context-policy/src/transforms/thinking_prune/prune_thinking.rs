//! prune thinking.
use super::*;

/// Strip `Thinking` parts from older messages and drop any messages
/// that become empty as a result.
///
/// # Examples
///
/// ```rust
/// use codetether_provider_core::{ContentPart, Message, Role};
/// use codetether_context_policy::transforms::thinking_prune::{
///     prune_thinking, KEEP_LAST_MESSAGES,
/// };
///
/// let thinking = ContentPart::Thinking { text: "Let me consider every option...".repeat(100), signature: None };
/// let reply = ContentPart::Text { text: "Here is my answer.".into() };
///
/// let mut msgs = vec![
///     Message { role: Role::Assistant, content: vec![thinking.clone(), reply.clone()] },
///     Message { role: Role::Assistant, content: vec![thinking.clone()] }, // thinking-only
/// ];
/// for i in 0..KEEP_LAST_MESSAGES + 1 {
///     msgs.push(Message {
///         role: Role::User,
///         content: vec![ContentPart::Text { text: format!("q{i}") }],
///     });
/// }
///
/// let stats = prune_thinking(&mut msgs);
/// assert!(stats.total_bytes_saved > 1000);
/// // Thinking-only message was removed entirely.
/// // Remaining first message retains the Text reply.
/// assert!(matches!(&msgs[0].content[..], [ContentPart::Text { .. }]));
/// ```
pub fn prune_thinking(messages: &mut Vec<Message>) -> ExperimentalStats {
    let mut stats = ExperimentalStats::default();
    let total = messages.len();
    if total <= KEEP_LAST_MESSAGES {
        return stats;
    }
    let eligible = total - KEEP_LAST_MESSAGES;

    // Prune parts in-place.
    for msg in messages[..eligible].iter_mut() {
        if msg.role != Role::Assistant {
            continue;
        }
        let before: usize = msg.content.iter().map(thinking_bytes).sum();
        msg.content
            .retain(|p| !matches!(p, ContentPart::Thinking { .. }));
        let after: usize = msg.content.iter().map(thinking_bytes).sum();
        let saved = before.saturating_sub(after);
        if saved > 0 {
            stats.total_bytes_saved += saved;
            stats.snippet_hits += 1;
        }
    }

    // Drop assistant messages that became empty. Indices in [0, eligible).
    let mut write = 0;
    for read in 0..messages.len() {
        let drop = read < eligible
            && messages[read].role == Role::Assistant
            && messages[read].content.is_empty();
        if !drop {
            if write != read {
                messages.swap(write, read);
            }
            write += 1;
        }
    }
    messages.truncate(write);

    stats
}
