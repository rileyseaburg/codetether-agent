//! Story-verification shell execution, retaining command-specific denial errors.
use super::Verification;
use anyhow::{Result, bail};
use codetether_application_registry::Application;
use codetether_bash_tool::BashTool;
use codetether_ralph_core::verification::{ShellExecutor, ShellOutput};
use codetether_tool_core::Tool;
use serde_json::json;
use std::path::Path;
#[async_trait::async_trait]
impl<A: Application> ShellExecutor for Verification<A> {
    async fn execute(&self, cwd: &Path, command: &str) -> Result<ShellOutput> {
        let args = json!({ "command": command, "cwd": cwd.display().to_string() });
        if let Some(blocked) = A::evaluate("bash", &args).await {
            bail!("command `{command}` blocked by policy: {}", blocked.output);
        }
        let result = BashTool::new().execute(args).await?;
        Ok(ShellOutput {
            success: result.success,
            output: result.output,
        })
    }
}
