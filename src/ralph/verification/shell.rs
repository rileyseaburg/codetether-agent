//! Application policy gate and BashTool adapter for shared verification.
use crate::tool::{Tool, bash::BashTool};
use anyhow::{Result, bail};
use codetether_ralph_core::verification::{ShellExecutor, ShellOutput};
use serde_json::json;
use std::path::Path;
pub(super) struct HostShell;
#[async_trait::async_trait]
impl ShellExecutor for HostShell {
    async fn execute(&self, cwd: &Path, command: &str) -> Result<ShellOutput> {
        let args = json!({ "command": command, "cwd": cwd.display().to_string() });
        if let Some(blocked) = crate::runtime_policy::evaluate_tool_invocation("bash", &args).await
        {
            bail!("command `{command}` blocked by policy: {}", blocked.output);
        }
        let result = BashTool::new().execute(args).await?;
        Ok(ShellOutput {
            success: result.success,
            output: result.output,
        })
    }
}
