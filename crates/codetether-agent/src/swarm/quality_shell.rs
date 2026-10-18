//! Policy-gated shell checks used by swarm branch probes.

#![cfg(test)]

use crate::tool::{Tool, bash::BashTool};
use anyhow::{Context, Result};
use serde_json::{Value, json};
use std::path::Path;

pub(super) const CARGO_CHECK_TIMEOUT_SECS: u64 = 45;

pub(super) fn cargo_check_quiet(cwd: Option<&Path>) -> Result<bool> {
    let args = args(cwd);
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .context("Failed to create runtime for swarm cargo check")?;
    runtime.block_on(async move {
        if let Some(blocked) = crate::runtime_policy::evaluate_tool_invocation("bash", &args).await
        {
            tracing::warn!(
                output = %blocked.output,
                "Swarm cargo check blocked by runtime policy"
            );
            return Ok(false);
        }
        let result = BashTool::new().execute(args).await?;
        Ok(result.success)
    })
}

fn args(cwd: Option<&Path>) -> Value {
    let mut args = json!({
        "command": "cargo check --quiet",
        "timeout": CARGO_CHECK_TIMEOUT_SECS,
    });
    if let Some(cwd) = cwd {
        args["cwd"] = json!(cwd.display().to_string());
    }
    args
}

#[cfg(test)]
#[path = "quality_shell_tests.rs"]
mod tests;
