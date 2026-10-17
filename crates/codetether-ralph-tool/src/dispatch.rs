//! Decode action arguments and dispatch the three Ralph operations.

use crate::{Host, RalphTool, params::Params};
use anyhow::{Context, Result};
use codetether_tool_core::ToolResult;
use serde_json::Value;
use std::path::PathBuf;
pub(crate) async fn execute<H: Host>(tool: &RalphTool<H>, params: Value) -> Result<ToolResult> {
    let p: Params = serde_json::from_value(params).context("Invalid params")?;
    let prd_path = PathBuf::from(p.prd_path.clone().unwrap_or_else(|| "prd.json".to_string()));
    match p.action.as_str() {
        "run" => crate::run::run(tool, p, prd_path).await,
        "status" => crate::status::read(&prd_path).await,
        "create-prd" => crate::create::create(p, &prd_path).await,
        _ => Ok(ToolResult::error(format!(
            "Unknown action: {}. Valid actions: run, status, create-prd",
            p.action
        ))),
    }
}
