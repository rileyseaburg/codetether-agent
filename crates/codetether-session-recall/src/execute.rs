//! Local evidence retrieval and optional answer synthesis.

use anyhow::Result;
use serde_json::json;

use codetether_tool_core::ToolResult;

use super::args::{RecallArgs, RecallMode};
use super::tool_struct::SessionRecallTool;

pub(super) async fn run<H: crate::Host>(
    tool: &SessionRecallTool<H>,
    value: serde_json::Value,
) -> Result<ToolResult> {
    let args = match RecallArgs::parse(&value) {
        Ok(args) => args,
        Err(error) => return Ok(ToolResult::error(error)),
    };
    let workspace = std::env::current_dir()?;
    let hits = H::search(
        &workspace,
        &args.query,
        crate::SearchOptions {
            session_id: args.session_id.as_deref(),
            excluded_session: None,
            limit: args.limit,
            minimum_score: 0.2,
        },
    )
    .await?;
    if hits.is_empty() {
        return Ok(super::faults::fault_result(
            codetether_context_policy::fault::Fault::NoMatch,
            "No indexed prior-session evidence matched the query.",
        ));
    }
    let evidence = crate::render::evidence(&hits, "Locally recalled session evidence:");
    if args.mode == RecallMode::Answer {
        return super::answer::run::<H>(tool, &args.query, &evidence, &hits).await;
    }
    Ok(ToolResult::success(evidence)
        .with_metadata("mode", json!("evidence"))
        .with_metadata("hits", json!(hits.len()))
        .truncate_to(codetether_tool_core::tool_output_budget()))
}
