//! Feature-gated native execution of shared refactor plugin policy.
use anyhow::Result;
use std::path::Path;

use codetether_refactor_guard::{GuardReport, GuardViolation};

#[cfg(feature = "tetherscript")]
struct Native;

#[cfg(feature = "tetherscript")]
impl codetether_refactor_guard::plugin::Runtime for Native {
    async fn execute(
        root: &Path,
        args: serde_json::Value,
    ) -> Result<codetether_tool_core::ToolResult> {
        use codetether_native_tetherscript::TetherScriptPluginTool;
        use codetether_tool_core::Tool;
        let tool = TetherScriptPluginTool::with_root(root.to_path_buf());
        tool.execute(args).await
    }
}

#[cfg(feature = "tetherscript")]
pub async fn apply(root: &Path, report: &GuardReport) -> Result<Option<GuardViolation>> {
    codetether_refactor_guard::plugin::apply::<Native>(root, report).await
}

#[cfg(not(feature = "tetherscript"))]
pub async fn apply(_root: &Path, _report: &GuardReport) -> Result<Option<GuardViolation>> {
    Ok(None)
}
