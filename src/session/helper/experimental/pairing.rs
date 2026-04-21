//! Invariant-repair pass: ensure every tool_call has its tool_result
//! and vice versa.
//!
//! Provider APIs (OpenAI Responses, Anthropic Messages, Gemini) all
//! enforce one of two invariants on conversation history:
//!
//! 1. Every assistant `tool_call` must be followed by a matching
//!    `tool_result` with the same `tool_call_id`.
//! 2. Every `tool_result` must reference a `tool_call_id` that exists
//!    earlier in the buffer.
//!
//! If a prior strategy in the [`super`] pipeline drops a message that
//! broke one of these invariants — for example, a middle-drop that
//! crosses a tool_call↔tool_result boundary — the provider rejects the
//! request outright (OpenAI: *"No tool output found for function call
//! call_XXX"*).
//!
//! This module runs **last** in [`super::apply_all`] and repairs
//! orphans in the safest way available:
//!
//! * **Orphan tool_call** (call with no matching result): inject a
//!   synthetic `Role::Tool` message immediately after it carrying a
//!   `[tool result unavailable: elided by context management]` payload.
//!   The call id is preserved so the model can correlate.
//! * **Orphan tool_result** (result whose call_id is nowhere earlier):
//!   drop the result. Providers that see an unreferenced result treat
//!   it as a malformed payload.
//!
//! # Always-on
//!
//! No config. This is a correctness pass — it can only make the buffer
//! more-valid, never less.

use super::ExperimentalStats;
use crate::provider::{ContentPart, Message, Role};
use std::collections::HashSet;

/// Repair orphaned tool_call/tool_result pairings.
///
/// Returns stats where `snippet_hits` counts the number of repairs
/// performed (injected synthetic results + dropped orphaned results)
/// and `total_bytes_saved` is 0 — this pass prioritizes correctness
/// over size.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::{ContentPart, Message, Role};
/// use codetether_agent::session::helper::experimental::pairing::repair_orphans;
///
/// // Assistant made a tool call, but the matching tool result message
/// // was dropped by an upstream pass.
/// let mut msgs = vec![
///     Message {
///         role: Role::Assistant,
///         content: vec![ContentPart::ToolCall {
///             id: "call_xyz".into(),
///             name: "browserctl".into(),
///             arguments: "{}".into(),
///             thought_signature: None,
///         }],
///     },
///     Message {
///         role: Role::User,
///         content: vec![ContentPart::Text {
///             text: "follow-up".into(),
///         }],
///     },
/// ];
///
/// let stats = repair_orphans(&mut msgs);
/// assert!(stats.snippet_hits >= 1);
/// assert_eq!(msgs.len(), 3);
/// assert_eq!(msgs[1].role, Role::Tool);
/// ```
pub fn repair_orphans(messages: &mut Vec<Message>) -> ExperimentalStats {
    let mut stats = ExperimentalStats::default();

    // Pass 1: inject synthetic tool results for orphan tool_calls.
    // Collect fulfilled ids in a forward walk; whenever we see a
    // ToolCall whose id is never fulfilled later, inject a placeholder.
    let fulfilled_ids = collect_fulfilled_ids(messages);
    let mut to_inject: Vec<(usize, String)> = Vec::new();
    for (idx, msg) in messages.iter().enumerate() {
        if msg.role != Role::Assistant {
            continue;
        }
        for part in &msg.content {
            if let ContentPart::ToolCall { id, .. } = part
                && !fulfilled_ids.contains(id)
            {
                to_inject.push((idx, id.clone()));
            }
        }
    }
    // Inject from the end so earlier indices stay valid.
    for (idx, call_id) in to_inject.into_iter().rev() {
        let placeholder = Message {
            role: Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: call_id,
                content: "[tool result unavailable: elided by context management]".into(),
            }],
        };
        messages.insert(idx + 1, placeholder);
        stats.snippet_hits += 1;
    }

    // Pass 2: drop tool_results that reference unknown call ids.
    // Build a prefix set of known call ids and remove any ToolResult
    // whose tool_call_id isn't in the prefix at its position.
    let mut known: HashSet<String> = HashSet::new();
    let mut drops: Vec<usize> = Vec::new();
    for (idx, msg) in messages.iter().enumerate() {
        // Record tool_call ids from this message for later results.
        for part in &msg.content {
            if let ContentPart::ToolCall { id, .. } = part {
                known.insert(id.clone());
            }
        }
        // If this message is tool-role with *every* result orphaned,
        // mark for drop.
        if msg.role == Role::Tool
            && !msg.content.is_empty()
            && msg.content.iter().all(|p| match p {
                ContentPart::ToolResult { tool_call_id, .. } => !known.contains(tool_call_id),
                _ => false,
            })
        {
            drops.push(idx);
        }
    }
    for idx in drops.into_iter().rev() {
        messages.remove(idx);
        stats.snippet_hits += 1;
    }

    stats
}

fn collect_fulfilled_ids(messages: &[Message]) -> HashSet<String> {
    let mut out = HashSet::new();
    for msg in messages {
        if msg.role != Role::Tool {
            continue;
        }
        for part in &msg.content {
            if let ContentPart::ToolResult { tool_call_id, .. } = part {
                out.insert(tool_call_id.clone());
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn call(id: &str) -> Message {
        Message {
            role: Role::Assistant,
            content: vec![ContentPart::ToolCall {
                id: id.into(),
                name: "t".into(),
                arguments: "{}".into(),
                thought_signature: None,
            }],
        }
    }

    fn result(id: &str) -> Message {
        Message {
            role: Role::Tool,
            content: vec![ContentPart::ToolResult {
                tool_call_id: id.into(),
                content: "ok".into(),
            }],
        }
    }

    fn user(t: &str) -> Message {
        Message {
            role: Role::User,
            content: vec![ContentPart::Text { text: t.into() }],
        }
    }

    #[test]
    fn well_formed_history_is_noop() {
        let mut msgs = vec![
            user("q"),
            call("a"),
            result("a"),
            call("b"),
            result("b"),
        ];
        let before = msgs.len();
        let stats = repair_orphans(&mut msgs);
        assert_eq!(stats.snippet_hits, 0);
        assert_eq!(msgs.len(), before);
    }

    #[test]
    fn orphan_call_gets_synthetic_result() {
        let mut msgs = vec![user("q"), call("a"), user("follow-up")];
        repair_orphans(&mut msgs);
        assert_eq!(msgs.len(), 4);
        assert_eq!(msgs[2].role, Role::Tool);
        let ContentPart::ToolResult {
            tool_call_id,
            content,
        } = &msgs[2].content[0]
        else {
            panic!("expected synthetic tool result");
        };
        assert_eq!(tool_call_id, "a");
        assert!(content.contains("unavailable"));
    }

    #[test]
    fn orphan_result_is_dropped() {
        let mut msgs = vec![user("q"), result("nonexistent"), user("next")];
        repair_orphans(&mut msgs);
        // The orphaned result is removed.
        assert_eq!(msgs.len(), 2);
        assert_eq!(msgs[0].role, Role::User);
        assert_eq!(msgs[1].role, Role::User);
    }

    #[test]
    fn mixed_orphan_and_valid_pairs() {
        let mut msgs = vec![
            user("q"),
            call("a"),        // will be orphaned
            call("b"),
            result("b"),
            user("mid"),
            result("c"),      // orphan — no prior call_c
        ];
        let stats = repair_orphans(&mut msgs);
        // One synthetic injection for "a", one drop for "c".
        assert_eq!(stats.snippet_hits, 2);
        // a's synthetic result should be inserted right after call("a").
        assert_eq!(msgs[1].role, Role::Assistant);
        assert_eq!(msgs[2].role, Role::Tool);
        let ContentPart::ToolResult { tool_call_id, .. } = &msgs[2].content[0] else {
            panic!();
        };
        assert_eq!(tool_call_id, "a");
    }
}
