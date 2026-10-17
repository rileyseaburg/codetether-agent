//! Terminal fallback for over-budget conversation history.
//!
//! [terminal_truncate_messages] retains the active tail, shrinks content,
//! and evicts old messages. Estimates and diagnostic reporting are host callbacks.
use codetether_provider_core::Message;
mod drop;
mod fit;
mod marker;
mod payload;
mod shrink;
mod shrink_caps;
mod shrink_part;

/// Drop or shrink history after summary-based compression is exhausted.
///
/// # Arguments
/// * `messages` - History to replace in place.
/// * `keep_last` - Minimum recent tail used to find the active user anchor.
/// * `target_tokens` - Full request budget, including system and tool overhead.
/// * `estimate` - Full request estimator for each candidate history.
/// * `on_fit` - Receives shrunk-part count, evicted-message count and budget
///   if either count is nonzero, before the final estimate.
///
/// # Returns
/// Saturating difference between the initial and final request estimates.
///
/// # Examples
/// ```
/// use codetether_context_policy::compression::terminal::terminal_truncate_messages;
/// let mut messages = vec![];
/// let removed = terminal_truncate_messages(&mut messages, 4, 100, |_| 0, |_, _, _| {});
/// assert_eq!(removed, 0);
/// assert!(messages.is_empty());
/// ```
pub fn terminal_truncate_messages(
    messages: &mut Vec<Message>,
    keep_last: usize,
    target_tokens: usize,
    estimate: impl Fn(&[Message]) -> usize,
    mut on_fit: impl FnMut(usize, usize, usize),
) -> usize {
    let before = estimate(messages);
    if messages.len() <= keep_last && before <= target_tokens {
        return 0;
    }

    let split_idx = if messages.len() > keep_last {
        crate::selection::active_tail::active_tail_start(messages, keep_last)
    } else {
        0
    };
    let dropped_prefix = split_idx > 0;
    let tail = if dropped_prefix {
        messages.split_off(split_idx)
    } else {
        std::mem::take(messages)
    };

    let mut new_messages = Vec::with_capacity(1 + tail.len());
    new_messages.push(marker::truncation_marker(dropped_prefix));
    new_messages.extend(tail);
    *messages = new_messages;

    fit::fit_retained(messages, target_tokens, &estimate, &mut on_fit);

    let after = estimate(messages);
    before.saturating_sub(after)
}
