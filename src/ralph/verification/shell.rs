use super::file::glob_exists;
use crate::tool::{Tool, bash::BashTool};
use anyhow::{Result, bail};
use serde_json::json;
use std::path::Path;

pub async fn run(
    root: &Path,
    command: &str,
    cwd: &Option<String>,
    expect_output_contains: &[String],
    expect_files_glob: &[String],
) -> Result<()> {
    let dir = cwd
        .as_ref()
        .map_or_else(|| root.to_path_buf(), |c| root.join(c));
    let args = json!({ "command": command, "cwd": dir.display().to_string() });
    if let Some(blocked) = crate::runtime_policy::evaluate_tool_invocation("bash", &args).await {
        bail!("command `{command}` blocked by policy: {}", blocked.output);
    }
    let result = BashTool::new().execute(args).await?;
    let combined = result.output;

    if !result.success {
        bail!("command `{command}` failed: {combined}");
    }
    for expected in expect_output_contains {
        if !combined.contains(expected) {
            bail!("command output did not contain `{expected}`");
        }
    }
    for pattern in expect_files_glob {
        if !glob_exists(&dir, pattern)? {
            bail!("command did not create files matching `{pattern}`");
        }
    }
    Ok(())
}
