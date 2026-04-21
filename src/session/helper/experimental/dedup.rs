//! Content-addressed deduplication of tool-result blocks.
//!
//! Agentic loops re-read the same file and re-run the same searches many
//! times. Each duplicate tool output costs full input tokens on every
//! subsequent turn. This module detects exact content duplicates via
//! SHA-256 and replaces later copies with a short back-reference that
//! points at the first occurrence.
//!
//! # Safety
//!
//! *No information is lost* — the model can always ask the agent to
//! re-run the original tool call. The back-reference preserves the
//! `tool_call_id` of the first sighting so the model (or a human
//! auditing the transcript) can correlate the two.
//!
//! Only `ContentPart::ToolResult` blocks are considered; text,
//! assistant messages, tool-call arguments, images, and thinking blocks
//! are left untouched.
//!
//! # Threshold
//!
//! Tool outputs smaller than [`MIN_DEDUP_BYTES`] are left alone — the
//! marker itself would be nearly as long as the content, and small
//! outputs typically carry disambiguating structure (e.g. `"ok"` vs
//! `"error"`).

use sha2::{Digest, Sha256};
use std::collections::HashMap;

use super::ExperimentalStats;
use crate::provider::{ContentPart, Message};

/// Tool outputs shorter than this byte count are never deduplicated.
/// Chosen so the back-reference marker is always shorter than the
/// content it replaces.
pub const MIN_DEDUP_BYTES: usize = 256;

/// Replace duplicate tool-result contents with a back-reference marker.
///
/// Walks `messages` in order. The first tool result for a given content
/// hash is kept verbatim; every subsequent identical result is replaced
/// with a marker of the form:
///
/// ```text
/// [DEDUP] identical to tool_call_id=<first-id> (<N> bytes)
/// ```
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::{ContentPart, Message, Role};
/// use codetether_agent::session::helper::experimental::dedup::dedup_tool_outputs;
///
/// let big = "x".repeat(1024);
/// let mut msgs = vec![
///     Message {
///         role: Role::Tool,
///         content: vec![ContentPart::ToolResult {
///             tool_call_id: "a".into(),
///             content: big.clone(),
///         }],
///     },
///     Message {
///         role: Role::Tool,
///         content: vec![ContentPart::ToolResult {
///             tool_call_id: "b".into(),
///             content: big,
///         }],
///     },
/// ];
///
/// let stats = dedup_tool_outputs(&mut msgs);
/// assert_eq!(stats.dedup_hits, 1);
/// assert!(stats.total_bytes_saved > 512);
///
/// // First copy is preserved intact.
/// if let ContentPart::ToolResult { content, .. } = &msgs[0].content[0] {
///     assert_eq!(content.len(), 1024);
/// } else {
///     panic!("expected tool result");
/// }
///
/// // Second copy is now a back-reference pointing at call_a.
/// if let ContentPart::ToolResult { content, .. } = &msgs[1].content[0] {
///     assert!(content.starts_with("[DEDUP]"));
///     assert!(content.contains("tool_call_id=a"));
/// } else {
///     panic!("expected tool result");
/// }
/// ```
pub fn dedup_tool_outputs(messages: &mut [Message]) -> ExperimentalStats {
    let mut seen: HashMap<[u8; 32], String> = HashMap::new();
    let mut stats = ExperimentalStats::default();

    for msg in messages.iter_mut() {
        for part in msg.content.iter_mut() {
            let ContentPart::ToolResult {
                tool_call_id,
                content,
            } = part
            else {
                continue;
            };
            if content.len() < MIN_DEDUP_BYTES {
                continue;
            }
            let hash = Sha256::digest(content.as_bytes()).into();
            match seen.get(&hash) {
                Some(first_id) => {
                    let marker = format!(
                        "[DEDUP] identical to tool_call_id={first_id} ({} bytes)",
                        content.len()
                    );
                    let saved = content.len().saturating_sub(marker.len());
                    *content = marker;
                    stats.dedup_hits += 1;
                    stats.total_bytes_saved += saved;
                }
                None => {
                    seen.insert(hash, tool_call_id.clone());
                }
            }
        }
    }

    stats
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tool_msg(id: &str, content: &str) -> Message {
        Message {
            role: crate::provider::Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: id.into(),
                content: content.into(),
            }],
        }
    }

    #[test]
    fn short_outputs_are_not_deduplicated() {
        let mut msgs = vec![tool_msg("a", "ok"), tool_msg("b", "ok")];
        let stats = dedup_tool_outputs(&mut msgs);
        assert_eq!(stats.dedup_hits, 0);
    }

    #[test]
    fn distinct_outputs_are_preserved() {
        let mut msgs = vec![
            tool_msg("a", &"x".repeat(1024)),
            tool_msg("b", &"y".repeat(1024)),
        ];
        let stats = dedup_tool_outputs(&mut msgs);
        assert_eq!(stats.dedup_hits, 0);
        assert_eq!(stats.total_bytes_saved, 0);
    }

    #[test]
    fn three_way_dedup_references_first_sighting() {
        let big = "z".repeat(2048);
        let mut msgs = vec![
            tool_msg("first", &big),
            tool_msg("second", &big),
            tool_msg("third", &big),
        ];
        let stats = dedup_tool_outputs(&mut msgs);
        assert_eq!(stats.dedup_hits, 2);

        for idx in [1, 2] {
            let ContentPart::ToolResult { content, .. } = &msgs[idx].content[0] else {
                panic!("expected tool result");
            };
            assert!(content.contains("tool_call_id=first"));
        }
    }
}
