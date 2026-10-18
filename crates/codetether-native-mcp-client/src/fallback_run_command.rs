//! Execute translated fallback command arguments through the native sandbox.
use anyhow::Result;
use codetether_bash_tool::BashTool;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;
pub fn execute(bash_args: Value) -> Result<ToolResult> {
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current()
            .block_on(async { BashTool::new().execute(bash_args).await })
    })
}
