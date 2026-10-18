//! Quality-gate shell execution, retaining unsuccessful policy-denial output.
use super::Quality;
use codetether_application_registry::Application;
use codetether_bash_tool::BashTool;
use codetether_ralph_core::verification::{ShellExecutor, ShellOutput};
use codetether_tool_core::Tool;
use serde_json::json;
use std::path::Path;
#[async_trait::async_trait]
impl<A: Application> ShellExecutor for Quality<A> {
    async fn execute(&self, cwd: &Path, command: &str) -> anyhow::Result<ShellOutput> {
        let args = json!({
            "command": command,
            "cwd": cwd.display().to_string(),
        });
        if let Some(blocked) = A::evaluate("bash", &args).await {
            return Ok(ShellOutput {
                success: false,
                output: blocked.output,
            });
        }
        let result = BashTool::new().execute(args).await?;
        Ok(ShellOutput {
            success: result.success,
            output: result.output,
        })
    }
}
