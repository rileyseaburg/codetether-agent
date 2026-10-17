//! Apply validated operations in their original order after confirmation.
use super::{ToolResult, load::FileContent};
use anyhow::Result;
use tokio::fs;
pub(super) async fn apply(
    file_contents: Vec<FileContent>,
    previews_len: usize,
) -> Result<ToolResult> {
    // Apply all changes
    for (path, content, old_string, new_string) in file_contents {
        let new_content = content.replacen(&old_string, &new_string, 1);
        fs::write(&path, &new_content).await?;
    }

    Ok(ToolResult::success(format!(
        "✓ Applied {} file changes",
        previews_len
    )))
}
