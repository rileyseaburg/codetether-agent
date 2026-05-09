use super::estimation::estimate_tokens;
use crate::provider::{ContentPart, Message};

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
