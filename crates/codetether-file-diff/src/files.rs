//! files for file comparisons.
use super::*;
pub(super) async fn execute(args: &Value) -> Result<ToolResult> {
    // File comparison mode
    let file1 = match args["file1"].as_str() {
        Some(f) => f,
        None => {
            return Ok(ToolResult::structured_error(
                "INVALID_ARGUMENT",
                "diff",
                "file1 is required for file comparison (or use git=true)",
                Some(vec!["file1"]),
                Some(json!({"file1": "old.txt", "file2": "new.txt"})),
            ));
        }
    };
    let file2 = match args["file2"].as_str() {
        Some(f) => f,
        None => {
            return Ok(ToolResult::structured_error(
                "INVALID_ARGUMENT",
                "diff",
                "file2 is required for file comparison",
                Some(vec!["file2"]),
                Some(json!({"file1": file1, "file2": "new.txt"})),
            ));
        }
    };

    // Use system diff command for better output
    let output = tokio::process::Command::new("diff")
        .arg("-u")
        .arg(format!("--label={}", file1))
        .arg(format!("--label={}", file2))
        .arg(file1)
        .arg(file2)
        .output()
        .await?;

    let diff = String::from_utf8_lossy(&output.stdout);
    if diff.is_empty() && output.status.success() {
        Ok(ToolResult::success("Files are identical"))
    } else {
        Ok(ToolResult::success(diff.to_string()))
    }
}
