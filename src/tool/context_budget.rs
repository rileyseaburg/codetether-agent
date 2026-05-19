//! `context_budget`: report context window budget status.
//!
//! Returns current L_working, L_effective, recently elided ranges,
//! and the session's bucket projection. No provider call needed —
//! pure read from the session's summary index and page sidecar.

use super::context_helpers::load_latest_session;
use super::{Tool, ToolResult};
use crate::session::pages::PageKind;
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};

/// Context budget tool.
pub struct ContextBudgetTool;

#[async_trait]
impl Tool for ContextBudgetTool {
    fn id(&self) -> &str {
        "context_budget"
    }
    fn name(&self) -> &str {
        "ContextBudget"
    }

    fn description(&self) -> &str {
        "Report the current context window budget: messages count, \
         cached summaries, page distribution, and remaining capacity. \
         Call this to understand how much room is left before the next \
         compression or reset. Returns L_working (current transcript \
         length) and L_effective (with summarization multiplier)."
    }

    fn parameters(&self) -> Value {
        json!({ "type": "object", "properties": {} })
    }

    async fn execute(&self, _args: Value) -> Result<ToolResult> {
        let session = match load_latest_session().await {
            Ok(Some(s)) => s,
            Ok(None) => return Ok(ToolResult::error("No active session found.")),
            Err(e) => return Ok(ToolResult::error(&format!("Failed to load session: {e}"))),
        };
        let total = session.messages.len();
        let cached = session.summary_index.len();
        let pinned = session
            .pages
            .iter()
            .filter(|p| matches!(p, PageKind::Constraint | PageKind::Bootstrap))
            .count();
        let output = format!(
            "L_working={total} messages, cached_summaries={cached}, \
             pinned_pages={pinned}, L_effective≈{eff}",
            eff = total + cached * 50,
        );
        Ok(ToolResult::success(output))
    }
}
