//! Budget-aware session flattening for recall.

use codetether_provider_core::Message;
use codetether_rlm::RlmChunker;

/// Maximum estimated tokens per session for recall.
const RECALL_TOKEN_BUDGET: usize = 30_000;

/// Convert messages to a flat recall string with default budget.
/// # Arguments
/// Receives an ordered provider transcript.
/// # Returns
/// Rendered text and whether any content was omitted.
/// # Examples
/// ```
/// assert_eq!(codetether_recall_context::flatten_messages(&[]), (String::new(), false));
/// ```
pub fn flatten_messages(messages: &[Message]) -> (String, bool) {
    flatten_messages_with_budget(messages, RECALL_TOKEN_BUDGET)
}

/// Budget-aware flattening with a running token count (O(N), not O(N²)).
/// # Arguments
/// Receives an ordered transcript and its estimated token budget.
/// # Returns
/// Rendered text and a truncation flag, preserving source message indices.
/// # Examples
/// ```
/// assert_eq!(codetether_recall_context::flatten_messages_with_budget(&[], 0), (String::new(), false));
/// ```
pub fn flatten_messages_with_budget(messages: &[Message], budget: usize) -> (String, bool) {
    // Reserve a sane amount up front (the string still grows as needed). A
    // bogus `budget` must never turn this hint into a multi-GiB reservation.
    let mut out = String::with_capacity(budget.saturating_mul(4).min(1024 * 1024));
    let mut tokens = 0usize;
    let mut truncated = false;
    let sep = "\n---\n\n";
    let sep_tok = RlmChunker::estimate_tokens(sep);
    let marker = "[...truncated]\n";
    let marker_tok = RlmChunker::estimate_tokens(marker);

    for (idx, m) in messages.iter().enumerate() {
        let role = codetether_context_policy::transcript::role_label(m.role);
        let hdr = format!("[{idx} {role}]\n");
        let hdr_tok = RlmChunker::estimate_tokens(&hdr);

        if tokens + hdr_tok > budget {
            truncated = true;
            break;
        }
        out.push_str(&hdr);
        tokens += hdr_tok;

        truncated |= super::parts::append(
            &mut out,
            &mut tokens,
            &m.content,
            budget,
            marker,
            marker_tok,
        );

        if tokens + sep_tok > budget {
            truncated = true;
            break;
        }
        out.push_str(sep);
        tokens += sep_tok;
    }

    (out, truncated)
}
