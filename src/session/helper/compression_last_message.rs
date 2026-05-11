//! Quality gate + chunker fallback for the oversized-last-message path.

use crate::rlm::{RlmChunker, RlmResult};
use crate::session::index_produce::summary_gate::try_bounded_summary;

/// Either wrap a gated RLM summary into the documented "Original
/// message: N tokens, compressed via RLM" shape, or fall back to
/// chunker compression of the raw input.
pub(super) fn wrap_or_chunk(
    result: RlmResult,
    target_tokens: usize,
    original: &str,
    msg_tokens: usize,
    prefix_chars: usize,
) -> String {
    if let Some(body) = try_bounded_summary(&result, target_tokens) {
        let prefix: String = original.chars().take(prefix_chars).collect();
        return format!(
            "[Original message: {msg_tokens} tokens, compressed via RLM]\n\n{body}\n\n---\nOriginal request prefix:\n{prefix}"
        );
    }
    tracing::warn!(
        input_tokens = result.stats.input_tokens,
        output_tokens = result.stats.output_tokens,
        "RLM: Last-message output degraded; using chunker fallback"
    );
    RlmChunker::compress(original, target_tokens, None)
}
