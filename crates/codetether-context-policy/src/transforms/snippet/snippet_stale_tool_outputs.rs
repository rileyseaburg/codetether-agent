//! snippet stale tool outputs.
use super::*;

/// Replace the middle of stale oversized tool outputs with a short
/// `[...elided...]` marker.
///
/// # Examples
///
/// ```rust
/// use codetether_provider_core::{ContentPart, Message, Role};
/// use codetether_context_policy::transforms::snippet::{
///     snippet_stale_tool_outputs, RECENCY_WINDOW,
/// };
///
/// // Build a history with one oversized tool output followed by enough
/// // recent messages to push it outside the recency window.
/// let oversized = "A".repeat(10_000);
/// let mut msgs = vec![Message {
///     role: Role::Tool,
///     content: vec![ContentPart::ToolResult {
///         tool_call_id: "old".into(),
///         content: oversized,
///     }],
/// }];
/// for i in 0..RECENCY_WINDOW + 2 {
///     msgs.push(Message {
///         role: Role::User,
///         content: vec![ContentPart::Text {
///             text: format!("filler {i}"),
///         }],
///     });
/// }
///
/// let stats = snippet_stale_tool_outputs(&mut msgs);
/// assert_eq!(stats.snippet_hits, 1);
/// assert!(stats.total_bytes_saved > 5_000);
///
/// // Marker is present and head/tail are preserved.
/// if let ContentPart::ToolResult { content, .. } = &msgs[0].content[0] {
///     assert!(content.contains("[...elided"));
///     assert!(content.starts_with("A"));
///     assert!(content.ends_with("A"));
/// } else {
///     panic!("expected tool result");
/// }
/// ```
pub fn snippet_stale_tool_outputs(messages: &mut [Message]) -> ExperimentalStats {
    let mut stats = ExperimentalStats::default();
    let total = messages.len();
    if total <= RECENCY_WINDOW {
        return stats;
    }
    let stale_upto = total - RECENCY_WINDOW;

    for msg in messages[..stale_upto].iter_mut() {
        for part in msg.content.iter_mut() {
            let ContentPart::ToolResult { content, .. } = part else {
                continue;
            };
            if content.len() < SNIPPET_THRESHOLD_BYTES {
                continue;
            }
            if content.contains("[...elided") || content.starts_with("[DEDUP]") {
                continue;
            }
            let original_len = content.len();
            let head_end = floor_char_boundary(content, HEAD_BYTES);
            let tail_start = ceil_char_boundary(content, original_len - TAIL_BYTES);
            let elided = tail_start - head_end;
            let mut rebuilt = String::with_capacity(HEAD_BYTES + TAIL_BYTES + 64);
            rebuilt.push_str(&content[..head_end]);
            rebuilt.push_str(&format!("\n[...elided {elided} bytes...]\n"));
            rebuilt.push_str(&content[tail_start..]);
            let saved = original_len.saturating_sub(rebuilt.len());
            *content = rebuilt;
            stats.snippet_hits += 1;
            stats.total_bytes_saved += saved;
        }
    }

    stats
}
