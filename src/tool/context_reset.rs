//! `context_reset`: agent-callable Lu et al. reset (arXiv:2510.06727).
//!
//! ## What it does
//!
//! The agent hands the harness a compressed summary of the session so
//! far. The tool records the reset decision in the audit log and
//! returns a marker [`ToolResult`] that the next turn's derivation can
//! pick up (via `DerivePolicy::Reset`) as the new anchor for the
//! context window.
//!
//! Unlike [`session_recall`], this tool does **not** read disk or run
//! RLM. The summary text is supplied by the model itself — that is the
//! whole point of Lu et al.'s semantic: the *policy* decides what to
//! keep, rather than an external summariser guessing.
//!
//! ## When the agent should call it
//!
//! * The transcript is getting long and the model wants to proactively
//!   fold it before the next tool call.
//! * The agent has just completed a sub-task and wants to hand-off a
//!   compact state to itself for the next phase.
//! * The user's next message is about to bring in a large payload
//!   (large file, long web page) and the agent wants headroom.
//!
//! ## Invariants
//!
//! * The tool never mutates [`Session::messages`]. The summary enters
//!   history naturally — as the tool's `ToolResult` on the next turn.
//! * The returned marker text always starts with `[CONTEXT RESET]` so
//!   Phase B's derivation policies can recognise it without an
//!   out-of-band signal.
//!
//! ## Examples
//!
//! ```rust
//! use codetether_agent::tool::context_reset::{ContextResetTool, format_reset_marker};
//!
//! // Sanity-check the stable marker format the next-turn derivation
//! // policy can look for.
//! let body = format_reset_marker("summary body", 42);
//! assert!(body.starts_with("[CONTEXT RESET]"));
//! assert!(body.contains("summary body"));
//! assert!(body.contains("bytes=42"));
//!
//! let _ = ContextResetTool;
//! ```

use super::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};

#[path = "context_reset_audit.rs"]
mod audit;

/// Marker prefix that identifies a `context_reset` tool output in the
/// transcript. Phase B derivations search for this literal when
/// deciding whether a user-visible summary has already landed.
pub const RESET_MARKER_PREFIX: &str = "[CONTEXT RESET]";

/// Shape the tool-result payload carrying the agent-authored summary.
///
/// Kept as a free function so the exact wire format can be unit-tested
/// in isolation from the [`Tool`] trait impl.
pub fn format_reset_marker(summary: &str, bytes: usize) -> String {
    format!(
        "{RESET_MARKER_PREFIX}\n\
         The agent requested a Lu et al. (arXiv:2510.06727) context \
         reset. Everything older than this point should be treated as \
         summarised by the text below. The canonical transcript is \
         preserved on disk; call `session_recall` for dropped \
         details.\n\
         \n\
         bytes={bytes}\n\
         \n\
         {summary}"
    )
}

/// Agent-callable Lu et al. reset tool.
pub struct ContextResetTool;

#[async_trait]
impl Tool for ContextResetTool {
    fn id(&self) -> &str {
        "context_reset"
    }

    fn name(&self) -> &str {
        "ContextReset"
    }

    fn description(&self) -> &str {
        "REQUEST A CONTEXT RESET (Lu et al., arXiv:2510.06727). Call this \
         when the active transcript is approaching the model's context \
         window and you want to compress it in your own words before \
         continuing. Pass a `summary` string distilling the task state, \
         decisions, open questions, and any must-remember details. The \
         tool records the reset and returns a `[CONTEXT RESET]` marker \
         that will become the anchor for the next turn. The canonical \
         chat history is preserved on disk — call `session_recall` if \
         you later need a specific detail you dropped."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "summary": {
                    "type": "string",
                    "description": "Compressed task state in your own words: \
                                    goal, key decisions, open questions, \
                                    must-remember details. This replaces \
                                    the earlier transcript as the anchor \
                                    for subsequent turns."
                }
            },
            "required": ["summary"]
        })
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let summary = match args["summary"].as_str() {
            Some(s) if !s.trim().is_empty() => s.trim().to_string(),
            _ => {
                return Ok(ToolResult::error(
                    "`summary` is required and must be non-empty",
                ));
            }
        };
        let bytes = summary.len();
        audit::log(bytes).await;
        Ok(ToolResult::success(format_reset_marker(&summary, bytes)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn execute_requires_non_empty_summary() {
        let tool = ContextResetTool;
        for summary in ["", "   "] {
            let result = tool.execute(json!({"summary": summary})).await.unwrap();
            assert!(!result.success);
        }
    }

    #[tokio::test]
    async fn execute_wraps_summary_with_marker() {
        let tool = ContextResetTool;
        let result = tool
            .execute(json!({"summary": "goal: ship it; open: review"}))
            .await
            .unwrap();
        assert!(result.success);
        assert!(result.output.starts_with(RESET_MARKER_PREFIX));
        assert!(result.output.contains("goal: ship it"));
    }

    #[test]
    fn format_reset_marker_has_stable_prefix_and_body() {
        let body = format_reset_marker("hello", 5);
        assert_eq!(RESET_MARKER_PREFIX, "[CONTEXT RESET]");
        assert!(body.starts_with(RESET_MARKER_PREFIX));
        assert!(body.contains("hello"));
        assert!(body.contains("bytes=5"));
        assert!(body.contains("session_recall"));
    }
}
