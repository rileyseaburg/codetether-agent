//! Tool trait implementation for `context_summarize`.

use super::super::context_helpers::load_latest_session;
use super::super::{Tool, ToolResult};
use super::logic::{lookup_cached, not_cached_msg, parse_range};
use super::schema;
use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

/// Summarize (read cached) or request summary for a turn range.
pub struct ContextSummarizeTool;

#[async_trait]
impl Tool for ContextSummarizeTool {
    fn id(&self) -> &str {
        "context_summarize"
    }
    fn name(&self) -> &str {
        "ContextSummarize"
    }

    fn description(&self) -> &str {
        "Get a cached summary for a range of conversation turns. \
         If a summary was already produced during context derivation, \
         returns it immediately. If not cached, reports the miss without \
         side effects. Pass `start` and `end` as 0-based turn indices \
         (end exclusive). Optionally set `target_tokens` (default 512)."
    }

    fn parameters(&self) -> Value {
        schema::parameters()
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        let (range, target) = match parse_range(&args) {
            Ok(parsed) => parsed,
            Err(err) => return Ok(err),
        };
        let session = match load_latest_session().await {
            Ok(Some(s)) => s,
            Ok(None) => return Ok(ToolResult::error("No active session.")),
            Err(e) => return Ok(ToolResult::error(&format!("Load failed: {e}"))),
        };
        match lookup_cached(&session, range) {
            Some(node) => Ok(ToolResult::success(format!(
                "Cached summary (tokens≈{}):\n{}",
                node.target_tokens, node.content,
            ))),
            None => Ok(ToolResult::success(not_cached_msg(
                range.start,
                range.end,
                target,
            ))),
        }
    }
}
