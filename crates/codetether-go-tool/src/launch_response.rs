//! Render the immediate launch acknowledgement and watch metadata.

use crate::launch_info::LaunchInfo;
use anyhow::Result;
use codetether_tool_core::ToolResult;
use serde_json::json;
pub(crate) fn render(info: LaunchInfo<'_>) -> Result<ToolResult> {
    let LaunchInfo {
        okr_id_str,
        task,
        resolved_model,
        max_iterations,
        working_dir,
        prd_filename,
        progress_filename,
    } = info;
    let output = format!(
        include_str!("launched.md"),
        okr_id_str = okr_id_str,
        task = task,
        resolved_model = resolved_model,
        max_iterations = max_iterations,
        working_dir = working_dir,
        prd_filename = prd_filename,
        progress_filename = progress_filename
    );

    Ok(ToolResult::success(output)
        .with_metadata("okr_id", json!(okr_id_str))
        .with_metadata("phase", json!("starting"))
        .with_metadata("prd_filename", json!(prd_filename))
        .with_metadata("progress_filename", json!(progress_filename))
        .with_metadata(
            "watch_hint",
            json!({
                "tool": "go",
                "args": {"action": "watch", "okr_id": okr_id_str}
            }),
        ))
}
