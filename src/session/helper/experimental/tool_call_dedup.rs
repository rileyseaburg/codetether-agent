//! Collapse redundant identical tool calls.
//!
//! When an agent re-issues the same tool call with the same arguments
//! (e.g. `read_file { path: "src/lib.rs" }` called at step 3 and again
//! at step 17), the *call* itself is wasted tokens — the earlier
//! response is already in history (and by this point usually already
//! deduplicated by [`super::dedup`]).
//!
//! This pass walks the buffer, hashes each `ContentPart::ToolCall` by
//! `(name, arguments)`, and replaces the *arguments* of later
//! duplicates with a short `[REPEAT of call_X]` marker. The tool's
//! result block for the repeat is left untouched so the model still
//! sees a consistent call→result pairing; dedup will have collapsed
//! the result body if the output was identical.
//!
//! # Safety
//!
//! * Only the **arguments** string is rewritten. Name, id, and
//!   thought_signature are preserved so every provider adapter still
//!   round-trips.
//! * Tool calls in the most recent [`KEEP_LAST_MESSAGES`] messages are
//!   never touched — the model is likely actively reasoning over them.
//! * Only triggers when arguments exceed [`MIN_COLLAPSE_BYTES`] so
//!   micro-arguments like `{}` stay readable.
//!
//! # Always-on
//!
//! No config flag.

use sha2::{Digest, Sha256};
use std::collections::HashMap;

use super::ExperimentalStats;
use crate::provider::{ContentPart, Message};

/// Keep tool-call arguments verbatim in this many trailing messages.
pub const KEEP_LAST_MESSAGES: usize = 8;

/// Tool-call arguments shorter than this are never collapsed.
pub const MIN_COLLAPSE_BYTES: usize = 64;

/// Collapse duplicate tool-call arguments into short back-references.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::{ContentPart, Message, Role};
/// use codetether_agent::session::helper::experimental::tool_call_dedup::{
///     collapse_duplicate_calls, KEEP_LAST_MESSAGES,
/// };
///
/// let args = serde_json::json!({
///     "path": "src/very/deeply/nested/module.rs",
///     "encoding": "utf-8",
/// }).to_string();
///
/// let call = |id: &str| Message {
///     role: Role::Assistant,
///     content: vec![ContentPart::ToolCall {
///         id: id.into(),
///         name: "read_file".into(),
///         arguments: args.clone(),
///         thought_signature: None,
///     }],
/// };
///
/// let mut msgs = vec![call("first"), call("second")];
/// for i in 0..KEEP_LAST_MESSAGES + 1 {
///     msgs.push(Message {
///         role: Role::User,
///         content: vec![ContentPart::Text { text: format!("q{i}") }],
///     });
/// }
///
/// let stats = collapse_duplicate_calls(&mut msgs);
/// assert!(stats.dedup_hits >= 1);
///
/// // Second call's arguments now reference the first.
/// let ContentPart::ToolCall { arguments, .. } = &msgs[1].content[0] else {
///     panic!();
/// };
/// assert!(arguments.starts_with("[REPEAT"));
/// assert!(arguments.contains("first"));
/// ```
pub fn collapse_duplicate_calls(messages: &mut [Message]) -> ExperimentalStats {
    let mut stats = ExperimentalStats::default();
    let total = messages.len();
    if total <= KEEP_LAST_MESSAGES {
        return stats;
    }
    let eligible = total - KEEP_LAST_MESSAGES;
    let mut seen: HashMap<[u8; 32], String> = HashMap::new();

    for msg in messages[..eligible].iter_mut() {
        for part in msg.content.iter_mut() {
            let ContentPart::ToolCall {
                id, name, arguments, ..
            } = part
            else {
                continue;
            };
            if arguments.len() < MIN_COLLAPSE_BYTES {
                continue;
            }
            if arguments.starts_with("[REPEAT") {
                continue;
            }
            let mut h = Sha256::new();
            h.update(name.as_bytes());
            h.update(b"\0");
            h.update(arguments.as_bytes());
            let key: [u8; 32] = h.finalize().into();
            match seen.get(&key) {
                Some(first_id) => {
                    let marker =
                        format!("[REPEAT of call_id={first_id}, {} bytes]", arguments.len());
                    let saved = arguments.len().saturating_sub(marker.len());
                    *arguments = marker;
                    stats.dedup_hits += 1;
                    stats.total_bytes_saved += saved;
                }
                None => {
                    seen.insert(key, id.clone());
                }
            }
        }
    }

    stats
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::Role;

    fn call(id: &str, name: &str, args: &str) -> Message {
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: id.into(),
                name: name.into(),
                arguments: args.into(),
                thought_signature: None,
            }],
        }
    }

    #[test]
    fn different_names_not_merged() {
        let args = "a".repeat(200);
        let mut msgs = vec![call("1", "read_file", &args), call("2", "write_file", &args)];
        for _ in 0..KEEP_LAST_MESSAGES + 1 {
            msgs.push(Message {
                role: Role::User,
                content: vec![ContentPart::Text { text: "q".into() }],
            });
        }
        let stats = collapse_duplicate_calls(&mut msgs);
        assert_eq!(stats.dedup_hits, 0);
    }

    #[test]
    fn small_args_preserved() {
        let mut msgs = vec![call("1", "noop", "{}"), call("2", "noop", "{}")];
        for _ in 0..KEEP_LAST_MESSAGES + 1 {
            msgs.push(Message {
                role: Role::User,
                content: vec![ContentPart::Text { text: "q".into() }],
            });
        }
        let stats = collapse_duplicate_calls(&mut msgs);
        assert_eq!(stats.dedup_hits, 0);
    }

    #[test]
    fn recent_calls_preserved() {
        let args = "z".repeat(200);
        let mut msgs = vec![call("1", "f", &args)];
        for _ in 0..KEEP_LAST_MESSAGES - 1 {
            msgs.push(Message {
                role: Role::User,
                content: vec![ContentPart::Text { text: "q".into() }],
            });
        }
        msgs.push(call("2", "f", &args));
        let stats = collapse_duplicate_calls(&mut msgs);
        assert_eq!(stats.dedup_hits, 0);
    }
}
