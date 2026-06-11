//! Policy-gated shell execution for Ralph quality checks.

use crate::tool::{Tool, bash::BashTool};
use anyhow::Result;
use serde_json::json;
use std::path::Path;

pub(super) struct QualityShellOutput {
    pub success: bool,
    pub combined: String,
}

pub(super) async fn run(cwd: &Path, command: &str) -> Result<QualityShellOutput> {
    let args = json!({
        "command": command,
        "cwd": cwd.display().to_string(),
    });
    if let Some(blocked) = crate::runtime_policy::evaluate_tool_invocation("bash", &args).await {
        return Ok(QualityShellOutput {
            success: false,
            combined: blocked.output,
        });
    }
    let result = BashTool::new().execute(args).await?;
    Ok(QualityShellOutput {
        success: result.success,
        combined: result.output,
    })
}
