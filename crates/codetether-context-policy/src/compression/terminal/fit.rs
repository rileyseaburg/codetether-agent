//! Sequence payload shrinking and message eviction before reporting.
use super::shrink::shrink_retained_payloads_to_budget;
use codetether_provider_core::Message;

/// Shrink retained payloads, then evict oldest messages, until the request
/// fits `target_tokens` (or nothing more can be removed).
pub(super) fn fit_retained(
    messages: &mut Vec<Message>,
    target_tokens: usize,
    estimate: &impl Fn(&[Message]) -> usize,
    on_fit: &mut impl FnMut(usize, usize, usize),
) {
    let shrunk = shrink_retained_payloads_to_budget(messages, target_tokens, estimate);
    let evicted = super::drop::drop_oldest_until_fits(messages, target_tokens, estimate);
    if shrunk + evicted > 0 {
        on_fit(shrunk, evicted, target_tokens);
    }
}
