//! Explicit slower RLM synthesis over locally retrieved evidence.

use anyhow::Result;
use std::sync::Arc;

use crate::tool::ToolResult;

pub(super) async fn run(
    tool: &super::tool_struct::SessionRecallTool,
    query: &str,
    evidence: &str,
    hits: &[crate::session::index::recall::hit::RecallHit],
) -> Result<ToolResult> {
    let sources = crate::session::index::recall::render::sources(hits);
    super::rlm_run::run_recall(
        evidence,
        &sources,
        query,
        Arc::clone(&tool.provider),
        &tool.model,
        &tool.config,
    )
    .await
}
