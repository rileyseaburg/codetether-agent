//! Confirmed file write and its resulting message.
use super::{ToolResult, diff::Preview, input::Input};
use anyhow::Result;
use std::time::Instant;
use tokio::fs;
pub(super) async fn apply(
    input: &Input,
    content: &str,
    new_content: &str,
    preview: &Preview,
) -> Result<ToolResult> {
    let start = Instant::now();

    // Calculate line range affected
    let lines_before = input.old_string.lines().count() as u32;
    let start_line = content[..content.find(input.old_string.as_str()).unwrap_or(0)]
        .lines()
        .count() as u32
        + 1;
    let end_line = start_line + lines_before.saturating_sub(1);

    // Write the file
    fs::write(&input.path, new_content).await?;

    let duration = start.elapsed();

    super::telemetry::record(input, preview, start_line, end_line, duration);
    Ok(ToolResult::success(format!(
        "✓ Changes applied to {}\n\nDiff:\n{}",
        input.path,
        preview.diff_output.trim()
    )))
}
