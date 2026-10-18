use anyhow::Result;
use std::path::Path;

use codetether_refactor_guard::{GuardReport, GuardViolation};

#[cfg(feature = "tetherscript")]
pub async fn apply(root: &Path, report: &GuardReport) -> Result<Option<GuardViolation>> {
    use crate::tool::{Tool, tetherscript::TetherScriptPluginTool};
    use serde_json::{Value, json};
    let hook = ".codetether/refactor_guard.tether";
    if !root.join(hook).exists() {
        return Ok(None);
    }
    let tool = TetherScriptPluginTool::with_root(root.to_path_buf());
    let args = json!({
        "path": hook,
        "hook": "should_continue",
        "args": [serde_json::to_string(report)?],
        "timeout_secs": 2
    });
    let result = tool.execute(args).await?;
    if !result.success {
        return Ok(Some(GuardViolation::new(hook, result.output)));
    }
    Ok(codetether_refactor_guard::read_decision(
        &result.metadata.get("value").cloned().unwrap_or(Value::Null),
    ))
}

#[cfg(not(feature = "tetherscript"))]
pub async fn apply(_root: &Path, _report: &GuardReport) -> Result<Option<GuardViolation>> {
    Ok(None)
}
