//! Validation, approval claiming, and dispatch for structured Git operations.

use super::{commit, execute_scope, ops, process};
use crate::tool::ToolResult;
use anyhow::{Result, anyhow};
use serde_json::Value;

pub(super) async fn run(mut args: Value) -> Result<ToolResult> {
    let op = args
        .get("op")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    if !matches!(
        op.as_str(),
        "status" | "diff" | "diff_staged" | "log" | "branch" | "show" | "commit"
    ) {
        return Ok(ToolResult::error(format!("Unknown git operation: {op}")));
    }
    execute_scope::bind_cwd(&mut args)?;
    if op == "commit" && args.get("message").and_then(Value::as_str).is_none() {
        return Ok(ToolResult::error("commit requires a 'message' argument"));
    }
    let cwd = args.get("cwd").and_then(Value::as_str).unwrap_or(".");
    if let Some(blocked) = crate::runtime_policy::evaluate_tool_invocation("git", &args).await {
        return Ok(blocked);
    }
    if let Err(error) = process::preflight(std::path::Path::new(cwd), op == "commit") {
        return Ok(ToolResult::error(error.to_string()));
    }
    crate::approval::use_once::claim("git", &args)
        .map_err(|error| anyhow!("approval claim failed: {error}"))?;
    if op == "commit" {
        commit::run_commit(&args).await
    } else {
        ops::run_readonly(&op, &args).await
    }
}
