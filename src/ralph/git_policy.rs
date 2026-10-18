//! Native policy binding and checkout compatibility entry point.
use anyhow::Result;
use std::{path::Path, process::Output};
pub(crate) struct Native;
#[async_trait::async_trait]
impl codetether_ralph_git::Host for Native {
    async fn check(args: serde_json::Value) -> Result<Option<crate::tool::ToolResult>> {
        Ok(crate::runtime_policy::evaluate_tool_invocation("bash", &args).await)
    }
}
pub(crate) fn checkout(cwd: &Path, branch: &str) -> Result<Output> {
    codetether_ralph_git::checkout::<Native>(cwd, branch)
}
