//! Terminal truncation: drop the oldest messages with no summary.

use crate::provider::{Message, ToolDefinition};
use crate::session::helper::token::estimate_request_tokens;

use super::shrink::shrink_retained_payloads_to_budget;
use super::terminal_marker::truncation_marker;

/// Drop everything older than the last `keep_last` messages in
/// `messages` and return an approximate count of the tokens removed.
///
/// Unlike [`compress_messages_keep_last`](super::compress_messages_keep_last)
/// this keeps **no** summary of the dropped prefix — the caller is
/// expected to have already attempted RLM-based compaction and exhausted
/// it. A `[CONTEXT TRUNCATED]` assistant marker is prepended so the model
/// is aware that older turns were silently removed.
///
/// # Returns
///
/// An approximate count of tokens removed (saturating subtraction).
/// Returns `0` when `messages.len() <= keep_last`, the request already
/// fits `target_tokens`, and no work is done.
pub(crate) fn terminal_truncate_messages(
    messages: &mut Vec<Message>,
    system_prompt: &str,
    tools: &[ToolDefinition],
    keep_last: usize,
    target_tokens: usize,
) -> usize {
    let before = estimate_request_tokens(system_prompt, messages, tools);
    if messages.len() <= keep_last && before <= target_tokens {
        return 0;
    }

    let split_idx = if messages.len() > keep_last {
        crate::session::context::active_tail::active_tail_start(messages, keep_last)
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
    new_messages.push(truncation_marker(dropped_prefix));
    new_messages.extend(tail);
    *messages = new_messages;

    let shrunk_parts =
        shrink_retained_payloads_to_budget(messages, system_prompt, tools, target_tokens);
    if shrunk_parts > 0 {
        tracing::warn!(
            shrunk_parts,
            target_tokens,
            after_tokens = estimate_request_tokens(system_prompt, messages, tools),
            "Terminal truncation shortened retained message payloads"
        );
    }

    let after = estimate_request_tokens(system_prompt, messages, tools);
    before.saturating_sub(after)
}
