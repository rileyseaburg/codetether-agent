//! `SessionRecallTool` struct and `Tool` trait implementation.

use crate::provider::Provider;
use crate::rlm::RlmConfig;
use crate::tool::{Tool, ToolResult};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;

const DEFAULT_SESSION_LIMIT: usize = 3;
/// Hard cap preventing OOM from unreasonably high `limit`.
const MAX_SESSION_LIMIT: usize = 5;

const DESCRIPTION: &str = "\
RECALL FROM YOUR OWN PAST SESSIONS. Call this whenever you see \
[AUTO CONTEXT COMPRESSION] and need specifics, or the user references \
something from earlier. Pass a natural-language `query`. Optionally \
pin `session_id` or widen `limit` (default 3, max 5). Distinct from \
`memory`: `memory` is curated notes; `session_recall` is the raw archive.";

/// RLM-backed recall tool over persisted session history.
pub struct SessionRecallTool {
    provider: Arc<dyn Provider>,
    model: String,
    config: RlmConfig,
}

impl SessionRecallTool {
    pub fn new(provider: Arc<dyn Provider>, model: String, config: RlmConfig) -> Self {
        Self {
            provider,
            model,
            config,
        }
    }
}

#[async_trait]
impl Tool for SessionRecallTool {
    fn id(&self) -> &str {
        "session_recall"
    }
    fn name(&self) -> &str {
        "SessionRecall"
    }
    fn description(&self) -> &str {
        DESCRIPTION
    }
    fn parameters(&self) -> serde_json::Value {
        super::schema::parameters()
    }

    async fn execute(&self, args: serde_json::Value) -> Result<ToolResult> {
        let query = match args["query"].as_str() {
            Some(q) if !q.trim().is_empty() => q.to_string(),
            _ => return Ok(ToolResult::error("query is required")),
        };
        let sid = args["session_id"].as_str().map(str::to_string);
        let limit = args["limit"]
            .as_u64()
            .map(|n| n as usize)
            .unwrap_or(DEFAULT_SESSION_LIMIT)
            .clamp(1, MAX_SESSION_LIMIT);

        let (ctx, sources) = match super::context::build_recall_context(sid, limit).await {
            Ok(ok) => ok,
            Err(e) => {
                return Ok(super::faults::fault_result(
                    super::faults::fault_from_error(&e),
                    format!("recall load failed: {e}"),
                ));
            }
        };
        if ctx.trim().is_empty() {
            return Ok(super::faults::fault_result(
                crate::session::Fault::NoMatch,
                "No prior session history found.",
            ));
        }
        super::rlm_run::run_recall(
            &ctx,
            &sources,
            &query,
            Arc::clone(&self.provider),
            &self.model,
            &self.config,
        )
        .await
    }
}
