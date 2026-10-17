//! prune low entropy.
use super::*;

/// Prune low-information whitespace and formatting noise from older
/// assistant text parts.
///
/// # Examples
///
/// ```rust
/// use codetether_provider_core::{ContentPart, Message, Role};
/// use codetether_context_policy::transforms::lingua::{
///     prune_low_entropy, KEEP_LAST_MESSAGES,
/// };
///
/// let noisy = "Here is my   plan.\n\n\n\n...\n   \nAnd it works.  \n";
/// let mut msgs = vec![Message {
///     role: Role::Assistant,
///     content: vec![ContentPart::Text { text: noisy.into() }],
/// }];
/// for i in 0..KEEP_LAST_MESSAGES + 1 {
///     msgs.push(Message {
///         role: Role::User,
///         content: vec![ContentPart::Text {
///             text: format!("q{i}"),
///         }],
///     });
/// }
///
/// let stats = prune_low_entropy(&mut msgs);
/// assert!(stats.total_bytes_saved > 0);
///
/// let ContentPart::Text { text } = &msgs[0].content[0] else {
///     panic!();
/// };
/// assert!(!text.contains("   ")); // triple-space collapsed
/// assert!(!text.contains("\n\n\n")); // triple-newline collapsed
/// assert!(text.contains("Here is my plan."));
/// assert!(text.contains("And it works."));
/// ```
pub fn prune_low_entropy(messages: &mut [Message]) -> ExperimentalStats {
    let mut stats = ExperimentalStats::default();
    let total = messages.len();
    if total <= KEEP_LAST_MESSAGES {
        return stats;
    }
    let eligible = total - KEEP_LAST_MESSAGES;

    for msg in messages[..eligible].iter_mut() {
        if msg.role != Role::Assistant {
            continue;
        }
        for part in msg.content.iter_mut() {
            let ContentPart::Text { text } = part else {
                continue;
            };
            let original_len = text.len();
            let pruned = prune_text(text);
            if pruned.len() < original_len {
                let saved = original_len - pruned.len();
                *text = pruned;
                stats.total_bytes_saved += saved;
                stats.snippet_hits += 1;
            }
        }
    }

    stats
}
