//! Strip extended-thinking blocks from older messages.
//!
//! Modern reasoning models (Claude extended thinking, DeepSeek R1,
//! GPT-5 reasoning, Gemini thought summaries) emit `Thinking` content
//! parts that can be **10-100×** larger than the assistant's actual
//! reply. These blocks help the *current* turn's decision but carry
//! almost no value once the turn has produced its tool calls and the
//! loop has moved on — the final answer/action already reflects them.
//!
//! This module removes [`ContentPart::Thinking`] from every message
//! older than [`KEEP_LAST_MESSAGES`]. Recent thinking is preserved so
//! the model can still reference its own recent chain-of-thought.
//!
//! # Safety
//!
//! * Providers that inject thinking for correctness (cache-coherent
//!   thought signatures on Gemini `ToolCall`) are unaffected — those
//!   signatures live on `ContentPart::ToolCall::thought_signature`,
//!   not on `Thinking` blocks.
//! * An assistant message whose *only* content was a thinking block
//!   becomes empty; such messages are removed entirely to keep the
//!   buffer a valid provider-consumable shape.
//!
//! # Always-on
//!
//! No config. Thinking blocks are known to be non-essential after the
//! turn completes; stripping them is the single highest-ROI shrink for
//! reasoning-heavy agent loops.

use super::ExperimentalStats;
use crate::provider::{ContentPart, Message, Role};

/// Keep thinking blocks in this many trailing messages.
pub const KEEP_LAST_MESSAGES: usize = 4;

/// Strip `Thinking` parts from older messages and drop any messages
/// that become empty as a result.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::{ContentPart, Message, Role};
/// use codetether_agent::session::helper::experimental::thinking_prune::{
///     prune_thinking, KEEP_LAST_MESSAGES,
/// };
///
/// let thinking = ContentPart::Thinking {
///     text: "Let me consider every option...".repeat(100),
/// };
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

fn thinking_bytes(p: &ContentPart) -> usize {
    match p {
        ContentPart::Thinking { text } => text.len(),
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recent_thinking_preserved() {
        let mut msgs = vec![Message {
            role: Role::Assistant,
            content: vec![ContentPart::Thinking {
                text: "x".repeat(5000),
            }],
        }];
        let stats = prune_thinking(&mut msgs);
        assert_eq!(stats.total_bytes_saved, 0);
    }

    #[test]
    fn empty_assistant_dropped() {
        let mut msgs = vec![Message {
            role: Role::Assistant,
            content: vec![ContentPart::Thinking {
                text: "x".repeat(1000),
            }],
        }];
        for i in 0..KEEP_LAST_MESSAGES + 1 {
            msgs.push(Message {
                role: Role::User,
                content: vec![ContentPart::Text {
                    text: format!("q{i}"),
                }],
            });
        }
        let before = msgs.len();
        prune_thinking(&mut msgs);
        assert_eq!(msgs.len(), before - 1);
    }

    #[test]
    fn user_thinking_untouched() {
        // Users can't really emit thinking, but defense-in-depth.
        let mut msgs = vec![Message {
            role: Role::User,
            content: vec![ContentPart::Thinking {
                text: "user-thought".repeat(100),
            }],
        }];
        for i in 0..KEEP_LAST_MESSAGES + 1 {
            msgs.push(Message {
                role: Role::Assistant,
                content: vec![ContentPart::Text {
                    text: format!("r{i}"),
                }],
            });
        }
        let stats = prune_thinking(&mut msgs);
        assert_eq!(stats.total_bytes_saved, 0);
    }
}
