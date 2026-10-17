//! git for file comparisons.
use super::*;
pub(super) async fn execute(args: &Value, staged: bool, context: u64) -> Result<ToolResult> {
    // Git diff mode
    let mut cmd = tokio::process::Command::new("git");
    cmd.arg("diff");

    if staged {
        cmd.arg("--staged");
    }

    cmd.arg(format!("-U{}", context));

    if let Some(file) = args["file1"].as_str() {
        cmd.arg("--").arg(file);
    }

    let output = cmd.output().await?;

    if output.status.success() {
        let diff = String::from_utf8_lossy(&output.stdout);
        if diff.is_empty() {
            Ok(ToolResult::success("No changes detected"))
        } else {
            Ok(ToolResult::success(diff.to_string()))
        }
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Ok(ToolResult::error(format!("Git diff failed: {}", error)))
    }
}
