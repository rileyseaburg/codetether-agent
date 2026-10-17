//! Experimental context-management strategies applied before RLM compaction.
//!
//! The agentic loop re-sends the entire conversation every step, which
//! means two structural wastes dominate token usage:
//!
//! 1. **Duplicate tool outputs.** Agents frequently re-read the same
//!    file, re-run the same `ls`, or re-grep for the same pattern
//!    across many steps. The verbatim content appears multiple times
//!    in the history. See [`dedup`].
//! 2. **Stale oversized tool outputs.** A 40 KB `read_file` result from
//!    step 2 is rarely relevant at step 30, yet it still costs full
//!    input tokens every turn. See [`snippet`].
//!
//! Both strategies are **lossy** in the strict sense but preserve
//! referenceability: the model can always ask the agent to re-run the
//! original tool call if it needs the full output back.
//!
//! # Composition
//!
//! [`apply_all`] runs every strategy in a fixed order against the live
//! [`Message`] buffer, mutating in place. Callers (the two prompt loops)
//! invoke it immediately before
//! [`enforce_context_window`](super::compression::enforce_context_window)
//! so the RLM compaction pass sees the already-shrunken buffer. The
//! returned [`ExperimentalStats`] is logged at `info` level for
//! observability.
//!
//! # Default-on, no config
//!
//! The conservative strategies in [`apply_all`] are always active —
//! there is intentionally no env flag to disable them. More aggressive
//! transforms should stay opt-in until they prove they do not erase
//! still-relevant chat state.
//!
//! # Examples
//!
//! ```rust
//! use codetether_agent::provider::{ContentPart, Message, Role};
//! use codetether_agent::session::helper::experimental::apply_all;
//!
//! let tool_result = ContentPart::ToolResult {
//!     tool_call_id: "call_a".into(),
//!     content: "file contents: hello world".repeat(40),
//! };
//! let duplicate = ContentPart::ToolResult {
//!     tool_call_id: "call_b".into(),
//!     content: "file contents: hello world".repeat(40),
//! };
//!
//! let mut msgs = vec![
//!     Message { role: Role::Tool, content: vec![tool_result] },
//!     Message { role: Role::Tool, content: vec![duplicate] },
//! ];
//!
//! let stats = apply_all(&mut msgs);
//! assert!(stats.total_bytes_saved > 0);
//! assert!(stats.dedup_hits >= 1);
//! ```

pub mod dedup;
pub mod lingua;
pub mod pairing;
#[allow(dead_code)]
pub mod snippet;
#[allow(dead_code)]
pub mod streaming_llm;
pub mod tetherscript_repair;
pub mod thinking_prune;
#[allow(dead_code)]
pub mod tool_call_dedup;

use crate::provider::Message;
pub use codetether_context_policy::transforms::ExperimentalStats;
/// Apply the shared pipeline and retain application-scoped logging.
pub fn apply_all(messages: &mut Vec<Message>) -> ExperimentalStats {
    let stats = codetether_context_policy::transforms::pipeline::apply_all(
        messages,
        tetherscript_repair::repair_reasoning,
    );
    if stats.total_bytes_saved > 0 {
        tracing::info!(
            dedup_hits = stats.dedup_hits,
            snippet_hits = stats.snippet_hits,
            bytes_saved = stats.total_bytes_saved,
            "experimental context strategies applied"
        );
    }

    stats
}

#[cfg(test)]
mod apply_all_tests;
#[cfg(test)]
mod apply_all_tool_history_tests;
