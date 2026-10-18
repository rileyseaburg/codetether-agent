//! Sum the token costs of all transcript content parts.
use crate::estimate_tokens_for_part;
use codetether_provider_core::Message;

/// Sum the estimated costs of all message content.
/// # Arguments
/// * `messages` — Ordered transcript whose parts should be counted.
/// # Returns
/// The sum across every part, or zero for an empty transcript.
/// # Examples
/// ```
/// assert_eq!(codetether_request_tokens::estimate_tokens_for_messages(&[]), 0);
/// ```
pub fn estimate_tokens_for_messages(messages: &[Message]) -> usize {
    messages
        .iter()
        .map(|m| {
            m.content
                .iter()
                .map(estimate_tokens_for_part)
                .sum::<usize>()
        })
        .sum()
}
