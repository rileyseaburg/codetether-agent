//! Preserve head and tail while truncating large outputs.

mod hint;
use crate::chunker::RlmChunker;
use hint::build_rlm_hint;
use tracing::info;

/// Truncate output to `max_tokens` with head/tail preservation.
///
/// Returns `(truncated_text, was_truncated, original_token_count)`.
///
/// # Arguments
/// Tool identity and arguments shape the hint; `max_tokens` is the text budget.
///
/// # Examples
/// ```
/// use codetether_rlm::smart_truncate;
/// let (text, truncated, _) = smart_truncate("hello", "read", &serde_json::json!({}), 100);
/// assert_eq!(text, "hello");
/// assert!(!truncated);
/// ```
pub fn smart_truncate(
    output: &str,
    tool_id: &str,
    tool_args: &serde_json::Value,
    max_tokens: usize,
) -> (String, bool, usize) {
    let estimated = RlmChunker::estimate_tokens(output);
    if estimated <= max_tokens {
        return (output.to_string(), false, estimated);
    }

    info!(
        tool = tool_id,
        original_tokens = estimated,
        max_tokens,
        "Smart truncating large output"
    );

    let max_chars = max_tokens * 4;
    let head_chars = (max_chars as f64 * 0.6) as usize;
    let tail_chars = (max_chars as f64 * 0.3) as usize;

    let head: String = output.chars().take(head_chars).collect();
    let tail: String = output
        .chars()
        .rev()
        .take(tail_chars)
        .collect::<String>()
        .chars()
        .rev()
        .collect();

    let omitted =
        estimated - RlmChunker::estimate_tokens(&head) - RlmChunker::estimate_tokens(&tail);
    let hint = build_rlm_hint(tool_id, tool_args, estimated);

    let truncated = format!(
        "{}\n\n[... {} tokens truncated ...]\n\n{}\n\n{}",
        head, omitted, hint, tail
    );
    (truncated, true, estimated)
}
