//! `context_budget`: report context window budget status.
//!
//! Returns current L_working, cached summary count, page pin count,
//! and a rough L_effective projection. No provider call needed —
//! pure read from the session's summary index and page sidecar.

use super::context_helpers::load_latest_session;
use super::{Tool, ToolResult};
use crate::session::pages::PageKind;
use anyhow::Result;
use async_trait::async_trait;
use serde_json::{Value, json};

const CACHED_SUMMARY_EFFECTIVE_MESSAGES: usize = 50;

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
        "Report current context budget signals: message count, cached \
         summaries, pinned pages, and a rough L_effective projection. \
         Call this to understand context pressure before compression."
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
            eff = total + cached * CACHED_SUMMARY_EFFECTIVE_MESSAGES,
        );
        Ok(ToolResult::success(output))
    }
}
