//! Deserialize calls, upgrade the registry and join nested results in order.
use super::{
    BatchTool, Registry, params::Params, policy as batch_policy, summary as batch_summary,
};
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
use serde_json::Value;
use std::sync::Arc;
pub(super) async fn execute<R: Registry>(tool: &BatchTool<R>, params: Value) -> Result<ToolResult> {
    let p: Params = serde_json::from_value(params).context("Invalid params")?;

    if p.calls.is_empty() {
        return Ok(ToolResult::error("No calls provided"));
    }

    // Get registry from weak reference
    let registry = {
        let guard = tool.registry.read().unwrap_or_else(|e| e.into_inner());
        match guard.as_ref() {
            Some(weak) => match weak.upgrade() {
                Some(arc) => arc,
                None => return Ok(ToolResult::error("Registry no longer available")),
            },
            None => return Ok(ToolResult::error("Registry not initialized")),
        }
    };

    // Execute all calls in parallel
    let futures: Vec<_> = p
        .calls
        .iter()
        .enumerate()
        .map(|(i, call)| {
            let tool_id = call.tool.clone();
            let args = call.args.clone();
            let registry = Arc::clone(&registry);

            async move { batch_policy::execute(i, tool_id, args, registry).await }
        })
        .collect();

    let results = futures::future::join_all(futures).await;

    Ok(batch_summary::build(results))
}
