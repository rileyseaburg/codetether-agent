use super::estimation::estimate_tokens;
use codetether_provider_core::{ContentPart, Message};

/// Truncate output at a UTF-8 boundary, preferring the last newline.
///
/// # Arguments
/// * `content` - Tool output.
/// * `max_chars` - Maximum byte offset before the appended truncation notice.
///
/// # Returns
/// Original text when it fits, otherwise a prefix and notice.
///
/// # Examples
/// ```
/// assert_eq!(codetether_swarm_core::context::truncate_single_result("hello", 10), "hello");
/// ```
pub fn truncate_single_result(content: &str, max_chars: usize) -> String {
    if content.len() <= max_chars {
        return content.to_string();
    }
    let safe_limit = safe_boundary(content, max_chars.min(content.len()));
    let break_point = content[..safe_limit].rfind('\n').unwrap_or(safe_limit);
    format!(
        "{}...\n\n[OUTPUT TRUNCATED: {} -> {} chars to fit context limit]",
        &content[..break_point],
        content.len(),
        break_point
    )
}

/// Shorten tool-result content that exceeds the estimated token limit.
///
/// # Arguments
/// * `messages` - Messages mutated in place.
/// * `max_tokens_per_result` - Per-tool-result token threshold.
///
/// # Returns
/// The number of results whose byte length decreased.
///
/// # Examples
/// ```
/// assert_eq!(codetether_swarm_core::context::truncate_large_tool_results(&mut [], 100), 0);
/// ```
pub fn truncate_large_tool_results(
    messages: &mut [Message],
    max_tokens_per_result: usize,
) -> usize {
    let mut truncated_count = 0;
    let char_limit = max_tokens_per_result * 3;
    for message in messages {
        for part in &mut message.content {
            let ContentPart::ToolResult { content, .. } = part else {
                continue;
            };
            if estimate_tokens(content) <= max_tokens_per_result {
                continue;
            }
            let old_len = content.len();
            *content = truncate_single_result(content, char_limit);
            truncated_count += usize::from(content.len() < old_len);
        }
    }
    truncated_count
}

fn safe_boundary(content: &str, mut limit: usize) -> usize {
    while limit > 0 && !content.is_char_boundary(limit) {
        limit -= 1;
    }
    limit
}
