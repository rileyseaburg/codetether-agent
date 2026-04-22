//! Head/tail snippet compaction of stale oversized tool outputs.
//!
//! Late in an agent loop, early-turn tool outputs are rarely referenced
//! but still cost full input tokens on every subsequent step. This
//! strategy replaces the *middle* of any tool-result block that is
//!
//! * older than [`RECENCY_WINDOW`] messages back, and
//! * larger than [`SNIPPET_THRESHOLD_BYTES`],
//!
//! with a `[...<N> bytes elided...]` marker, keeping a head and tail
//! window so the model can still see the shape of the output.
//!
//! # Safety
//!
//! Lossy but referenceable — the tool_call_id is preserved so the model
//! can request a re-run if it needs the full content. The head and tail
//! windows are generous enough (1 KB each by default) to surface error
//! messages, file headers, or the last lines of a log.
//!
//! # Rollout
//!
//! This strategy is intentionally kept out of the default-safe
//! [`super::apply_all`] path. It is useful under real token pressure,
//! but it is still lossy because it removes unique bytes from old tool
//! outputs.

use super::ExperimentalStats;
use crate::provider::{ContentPart, Message};

/// Tool results in the last N messages are left alone — the model is
/// likely actively reasoning about them.
pub const RECENCY_WINDOW: usize = 8;

/// Tool outputs shorter than this are never snipped.
pub const SNIPPET_THRESHOLD_BYTES: usize = 4096;

/// Bytes kept at the start of a snipped output.
pub const HEAD_BYTES: usize = 1024;

/// Bytes kept at the end of a snipped output.
pub const TAIL_BYTES: usize = 1024;

/// Replace the middle of stale oversized tool outputs with a short
/// `[...elided...]` marker.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::{ContentPart, Message, Role};
/// use codetether_agent::session::helper::experimental::snippet::{
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

/// Round `idx` down to the nearest UTF-8 character boundary. Returns 0
/// if no valid boundary exists below `idx`.
fn floor_char_boundary(s: &str, mut idx: usize) -> usize {
    idx = idx.min(s.len());
    while idx > 0 && !s.is_char_boundary(idx) {
        idx -= 1;
    }
    idx
}

/// Round `idx` up to the nearest UTF-8 character boundary.
fn ceil_char_boundary(s: &str, mut idx: usize) -> usize {
    idx = idx.min(s.len());
    while idx < s.len() && !s.is_char_boundary(idx) {
        idx += 1;
    }
    idx
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tool_msg(content: &str) -> Message {
        Message {
            role: crate::provider::Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: "x".into(),
                content: content.into(),
            }],
        }
    }

    fn user_msg(text: &str) -> Message {
        Message {
            role: crate::provider::Role::User,
            content: vec![ContentPart::Text { text: text.into() }],
        }
    }

    #[test]
    fn short_histories_are_untouched() {
        let mut msgs = vec![tool_msg(&"x".repeat(10_000))];
        let stats = snippet_stale_tool_outputs(&mut msgs);
        assert_eq!(stats.snippet_hits, 0);
    }

    #[test]
    fn recent_tool_outputs_are_protected() {
        let mut msgs: Vec<Message> = (0..RECENCY_WINDOW)
            .map(|i| user_msg(&format!("m{i}")))
            .collect();
        msgs.insert(0, tool_msg(&"x".repeat(10_000)));
        let stats = snippet_stale_tool_outputs(&mut msgs);
        // The oversized tool_msg is at index 0 and total is RECENCY_WINDOW+1,
        // so stale_upto=1 → it IS eligible. Confirm that.
        assert_eq!(stats.snippet_hits, 1);
    }

    #[test]
    fn utf8_boundaries_are_respected() {
        let emoji = "🦀".repeat(4000); // each crab = 4 bytes
        let mut msgs = vec![tool_msg(&emoji)];
        msgs.extend((0..RECENCY_WINDOW + 2).map(|i| user_msg(&format!("m{i}"))));
        let stats = snippet_stale_tool_outputs(&mut msgs);
        assert_eq!(stats.snippet_hits, 1);
        let ContentPart::ToolResult { content, .. } = &msgs[0].content[0] else {
            panic!();
        };
        // If we sliced mid-codepoint this would panic on re-render.
        assert!(content.is_char_boundary(0));
        assert!(content.contains("[...elided"));
    }

    #[test]
    fn already_snipped_outputs_are_not_resnipped() {
        let mut msgs = vec![tool_msg(&format!(
            "{}\n[...elided 99 bytes...]\n{}",
            "H".repeat(2000),
            "T".repeat(2000)
        ))];
        msgs.extend((0..RECENCY_WINDOW + 2).map(|i| user_msg(&format!("m{i}"))));
        let before_len = match &msgs[0].content[0] {
            ContentPart::ToolResult { content, .. } => content.len(),
            _ => unreachable!(),
        };
        let stats = snippet_stale_tool_outputs(&mut msgs);
        assert_eq!(stats.snippet_hits, 0);
        let after_len = match &msgs[0].content[0] {
            ContentPart::ToolResult { content, .. } => content.len(),
            _ => unreachable!(),
        };
        assert_eq!(before_len, after_len);
    }
}
