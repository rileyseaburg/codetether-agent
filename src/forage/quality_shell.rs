//! Policy-gated shell execution for Forage quality checks.

use crate::tool::{Tool, bash::BashTool};
use anyhow::Result;
pub(super) use codetether_forage_core::quality::QualityShellOutput;
use serde_json::json;

pub(super) async fn run(command: &str) -> Result<QualityShellOutput> {
    let args = json!({ "command": command });
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
