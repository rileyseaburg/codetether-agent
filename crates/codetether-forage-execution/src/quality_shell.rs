//! Policy-gated shell execution for Forage quality checks.

use crate::Host;
use anyhow::Result;
use codetether_bash_tool::BashTool;
pub(crate) use codetether_forage_core::quality::QualityShellOutput;
use codetether_tool_core::Tool;
use serde_json::json;

pub(crate) async fn run<H: Host>(command: &str) -> Result<QualityShellOutput> {
    let args = json!({ "command": command });
    if let Some(blocked) = H::evaluate_tool_invocation("bash", &args).await {
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
