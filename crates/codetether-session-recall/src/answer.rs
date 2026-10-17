//! Explicit slower RLM synthesis over locally retrieved evidence.

use anyhow::Result;
use std::sync::Arc;

use codetether_tool_core::ToolResult;

pub(super) async fn run<H: crate::Host>(
    tool: &super::tool_struct::SessionRecallTool<H>,
    query: &str,
    evidence: &str,
    hits: &[crate::RecallHit],
) -> Result<ToolResult> {
    let sources = crate::render::sources(hits);
    super::rlm_run::run_recall::<H>(
        evidence,
        &sources,
        query,
        Arc::clone(&tool.provider),
        &tool.model,
        &tool.config,
    )
    .await
}
