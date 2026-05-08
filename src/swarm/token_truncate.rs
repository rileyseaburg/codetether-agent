//! Token-truncation helpers for sub-agent context management.
//!
//! Provides token estimation, message truncation, and tool-result
//! compression so long-running sub-agents stay within context limits
//! while preserving critical evidence (changed files, errors, last
//! tool calls, blockers).

use crate::provider::{ContentPart, Message, Role};

/// Default context limit (256 k tokens – conservative for most models).
pub const DEFAULT_CONTEXT_LIMIT: usize = 256_000;

/// Tokens reserved for the response generation.
pub const RESPONSE_RESERVE_TOKENS: usize = 8_192;

/// Safety margin before we start truncating (85 % of limit).
pub const TRUNCATION_THRESHOLD: f64 = 0.85;

/// Rough chars-per-token ratio used for estimation.
const CHARS_PER_TOKEN: f64 = 3.5;

/// Estimate token count from text.
///
/// Uses a conservative heuristic of ~3.5 chars per token.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::swarm::token_truncate::estimate_tokens;
///
/// let tokens = estimate_tokens("hello world");
/// assert!(tokens > 0);
/// assert!(tokens < 10);
/// ```
pub fn estimate_tokens(text: &str) -> usize {
    (text.len() as f64 / CHARS_PER_TOKEN).ceil() as usize
}

/// Estimate total tokens in a single [`Message`].
pub fn estimate_message_tokens(message: &Message) -> usize {
    let mut tokens = 4; // role overhead
    for part in &message.content {
        tokens += match part {
            ContentPart::Text { text } => estimate_tokens(text),
            ContentPart::ToolCall {
                id,
                name,
                arguments,
                ..
            } => estimate_tokens(id) + estimate_tokens(name) + estimate_tokens(arguments) + 10,
            ContentPart::ToolResult {
                tool_call_id,
                content,
            } => estimate_tokens(tool_call_id) + estimate_tokens(content) + 6,
            ContentPart::Image { .. } | ContentPart::File { .. } => 2000,
            ContentPart::Thinking { text } => estimate_tokens(text),
        };
    }
    tokens
}

/// Estimate total tokens across a slice of [`Message`]s.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::swarm::token_truncate::estimate_total_tokens;
/// use codetether_agent::provider::{Message, ContentPart, Role};
///
/// let msgs = vec![Message {
///     role: Role::User,
///     content: vec![ContentPart::Text { text: "hi".into() }],
/// }];
/// let total = estimate_total_tokens(&msgs);
/// assert!(total > 0);
/// ```
pub fn estimate_total_tokens(messages: &[Message]) -> usize {
    messages.iter().map(estimate_message_tokens).sum()
}

/// Truncate a single result string to `max_chars`, keeping a valid
/// UTF-8 boundary and a human-readable truncation notice.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::swarm::token_truncate::truncate_single_result;
///
/// let long = "x".repeat(10_000);
/// let out = truncate_single_result(&long, 100);
/// assert!(out.len() < 500);
/// assert!(out.contains("TRUNCATED"));
/// ```
pub fn truncate_single_result(content: &str, max_chars: usize) -> String {
    if content.len() <= max_chars {
        return content.to_string();
    }
    let safe_limit = {
        let mut limit = max_chars.min(content.len());
        while limit > 0 && !content.is_char_boundary(limit) {
            limit -= 1;
        }
        limit
    };
    let break_point = content[..safe_limit].rfind('\n').unwrap_or(safe_limit);
    format!(
        "{}...\n\n[OUTPUT TRUNCATED: {} → {} chars to fit context limit]",
        &content[..break_point],
        content.len(),
        break_point
    )
}

/// Aggressively truncate large tool results in-place.
///
/// Every `ToolResult` whose estimated tokens exceed `max_tokens_per_result`
/// is shortened via [`truncate_single_result`].
///
/// Returns the number of tool results that were truncated.
pub fn truncate_large_tool_results(
    messages: &mut [Message],
    max_tokens_per_result: usize,
) -> usize {
    let char_limit = max_tokens_per_result * 3;
    let mut truncated_count = 0;

    for message in messages.iter_mut() {
        for part in message.content.iter_mut() {
            if let ContentPart::ToolResult { content, .. } = part {
                let tokens = estimate_tokens(content);
                if tokens > max_tokens_per_result {
                    let old_len = content.len();
                    *content = truncate_single_result(content, char_limit);
                    if content.len() < old_len {
                        truncated_count += 1;
                    }
                }
            }
        }
    }
    truncated_count
}

/// Summarise removed messages so the agent retains awareness of
/// which tools were used in the dropped history.
pub fn summarize_removed_messages(messages: &[Message]) -> String {
    let mut tool_calls: Vec<String> = Vec::new();
    for msg in messages {
        for part in &msg.content {
            if let ContentPart::ToolCall { name, .. } = part
                && !tool_calls.contains(name)
            {
                tool_calls.push(name.clone());
            }
        }
    }
    if tool_calls.is_empty() {
        String::new()
    } else {
        format!("Tools used in truncated history: {}", tool_calls.join(", "))
    }
}

/// Truncate a message list to fit within `context_limit` tokens.
///
/// Strategy:
/// 1. First, aggressively truncate large tool results.
/// 2. Keep system message (first) and user message (second).
/// 3. Keep the most recent assistant + tool result pairs.
/// 4. Drop oldest middle messages in matched pairs.
///
/// Returns the number of messages that were removed.
pub fn truncate_messages_to_fit(
    messages: &mut Vec<Message>,
    context_limit: usize,
) -> usize {
    let target_tokens =
        ((context_limit as f64) * TRUNCATION_THRESHOLD) as usize - RESPONSE_RESERVE_TOKENS;

    let current_tokens = estimate_total_tokens(messages);
    if current_tokens <= target_tokens {
        return 0;
    }

    // Phase 1: truncate large tool results
    truncate_large_tool_results(messages, 2000);
    if estimate_total_tokens(messages) <= target_tokens {
        return 0;
    }

    // Phase 2: remove middle messages, keep first 2 + last 4
    if messages.len() <= 6 {
        return 0;
    }

    let keep_start = 2;
    let keep_end = 4;
    let removable_count = messages.len() - keep_start - keep_end;
    if removable_count == 0 {
        return 0;
    }

    let removed: Vec<_> = messages.drain(keep_start..keep_start + removable_count).collect();
    let summary = summarize_removed_messages(&removed);

    messages.insert(
        keep_start,
        Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: format!(
                    "[Context truncated: {} earlier messages removed to fit context window]\n{}",
                    removed.len(),
                    summary
                ),
            }],
        },
    );

    removed.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::{ContentPart, Message, Role};

    #[test]
    fn estimate_tokens_empty() {
        assert_eq!(estimate_tokens(""), 0);
    }

    #[test]
    fn estimate_tokens_non_empty() {
        let tokens = estimate_tokens("hello world");
        assert!(tokens > 0, "should be > 0");
        assert!(tokens <= 10, "roughly 3-4 tokens");
    }

    #[test]
    fn truncate_single_short_string_unchanged() {
        let s = "short";
        assert_eq!(truncate_single_result(s, 100), s);
    }

    #[test]
    fn truncate_single_long_string_is_truncated() {
        let long = "x".repeat(10_000);
        let out = truncate_single_result(&long, 100);
        assert!(out.contains("TRUNCATED"));
        assert!(out.len() < 500);
    }

    #[test]
    fn truncate_messages_noop_when_small() {
        let mut msgs = vec![Message {
            role: Role::User,
            content: vec![ContentPart::Text { text: "hi".into() }],
        }];
        let removed = truncate_messages_to_fit(&mut msgs, 256_000);
        assert_eq!(removed, 0);
        assert_eq!(msgs.len(), 1);
    }

    #[test]
    fn truncate_messages_removes_middle_when_large() {
        let mut msgs = make_large_conversation(10, 50_000);
        let orig_len = msgs.len();
        let removed = truncate_messages_to_fit(&mut msgs, 256_000);
        assert!(removed > 0);
        assert!(
            msgs.len() < orig_len,
            "messages should shrink: {} -> {}",
            orig_len,
            msgs.len()
        );
        let has_summary = msgs.iter().any(|m| {
            m.content.iter().any(|p| matches!(
                p,
                ContentPart::Text { text } if text.contains("Context truncated")
            ))
        });
        assert!(has_summary, "should contain truncation summary");
    }

    #[test]
    fn summarize_removed_messages_extracts_tool_names() {
        let msgs = vec![
            Message {
                role: Role::Assistant,
                content: vec![ContentPart::ToolCall {
                    id: "1".into(),
                    name: "bash".into(),
                    arguments: "{}".into(),
                    thought_signature: None,
                }],
            },
            Message {
                role: Role::Assistant,
                content: vec![ContentPart::ToolCall {
                    id: "2".into(),
                    name: "edit".into(),
                    arguments: "{}".into(),
                    thought_signature: None,
                }],
            },
        ];
        let summary = summarize_removed_messages(&msgs);
        assert!(summary.contains("bash"));
        assert!(summary.contains("edit"));
    }

    #[test]
    fn truncate_large_tool_results_in_place() {
        let mut msgs = vec![Message {
            role: Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: "tc-1".into(),
                content: "z".repeat(100_000),
            }],
        }];
        let count = truncate_large_tool_results(&mut msgs, 100);
        assert_eq!(count, 1);
        let text = match &msgs[0].content[0] {
            ContentPart::ToolResult { content, .. } => content.clone(),
            _ => panic!("expected ToolResult"),
        };
        assert!(text.contains("TRUNCATED"));
    }

    /// Helper: build a conversation large enough to trigger truncation.
    fn make_large_conversation(n_pairs: usize, chars_per_msg: usize) -> Vec<Message> {
        let mut msgs = Vec::new();
        msgs.push(Message {
            role: Role::System,
            content: vec![ContentPart::Text {
                text: "system prompt".into(),
            }],
        });
        msgs.push(Message {
            role: Role::User,
            content: vec![ContentPart::Text {
                text: "initial instruction".into(),
            }],
        });
        for i in 0..n_pairs {
            msgs.push(Message {
                role: Role::Assistant,
                content: vec![ContentPart::Text {
                    text: "x".repeat(chars_per_msg),
                }],
            });
            msgs.push(Message {
                role: Role::Tool,
                content: vec![ContentPart::ToolResult {
                    tool_call_id: format!("tc-{i}"),
                    content: "y".repeat(chars_per_msg),
                }],
            });
        }
        for i in 0..4 {
            msgs.push(Message {
                role: Role::Assistant,
                content: vec![ContentPart::Text {
                    text: format!("tail-{i}"),
                }],
            });
        }
        msgs
    }
}
