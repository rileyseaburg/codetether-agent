//! Read one file and validate its replacement target.
use super::{EditOperation, ToolResult};
use anyhow::{Context, Result};
use std::path::PathBuf;
use tokio::fs;
pub(super) type FileContent = (PathBuf, String, String, String);
pub(super) async fn load(edit: &EditOperation) -> Result<Result<FileContent, ToolResult>> {
    let path = PathBuf::from(&edit.file);

    if !path.exists() {
        return Ok(Err(ToolResult::error(format!(
            "File does not exist: {}",
            edit.file
        ))));
    }

    let content = fs::read_to_string(&path)
        .await
        .with_context(|| format!("Failed to read file: {}", edit.file))?;

    if let Some(error) = super::unique::validate(&content, edit) {
        return Ok(Err(error));
    }
    Ok(Ok((
        path,
        content,
        edit.old_string.clone(),
        edit.new_string.clone(),
    )))
}
