//! Validate the file, prepare its replacement, and write the edited content.
use super::{ToolResult, input::Input};
use anyhow::Result;
use serde_json::{Value, json};
use std::path::PathBuf;
use tokio::fs;
pub(super) async fn execute(params: Value) -> Result<ToolResult> {
    let Input {
        file_path,
        old_string,
        new_string,
        replace_all,
    } = match Input::parse(&params) {
        Ok(input) => input,
        Err(error) => return Ok(error),
    };
    let path = PathBuf::from(&file_path);
    if let Some(error) = super::path_check::check(&path, &file_path) {
        return Ok(error);
    }
    if old_string == new_string {
        return Ok(ToolResult::error(
            "oldString and newString must be different",
        ));
    }
    // Creating new file
    if old_string.is_empty() {
        fs::write(&path, &new_string).await?;
        return Ok(ToolResult::success(format!("Created file: {file_path}")));
    }
    let content = fs::read_to_string(&path).await?;

    let new_content = match super::prepare::prepare(
        &content,
        &old_string,
        &new_string,
        replace_all,
        &file_path,
    ) {
        Ok(content) => content,
        Err(error) => return Ok(error),
    };
    fs::write(&path, &new_content).await?;
    let old_lines = old_string.lines().count();
    let new_lines = new_string.lines().count();
    Ok(ToolResult::success(format!(
        "Edit applied: {old_lines} line(s) replaced with {new_lines} line(s) in {file_path}"
    ))
    .with_metadata("file", json!(file_path)))
}
