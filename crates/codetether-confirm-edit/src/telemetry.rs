//! Record confirmed file changes and tool execution metadata.
use super::{diff::Preview, input::Input};
use codetether_telemetry::{FileChange, TOOL_EXECUTIONS, ToolExecution, record_persistent};
use serde_json::json;
use std::time::Duration;
pub(super) fn record(
    input: &Input,
    preview: &Preview,
    start_line: u32,
    end_line: u32,
    duration: Duration,
) {
    // Record telemetry
    let file_change = FileChange::modify_with_diff(
        input.path.as_str(),
        preview.diff_output.as_str(),
        input.new_string.len(),
        Some((start_line, end_line)),
    );

    let mut exec = ToolExecution::start(
        "confirm_edit",
        json!({
            "path": input.path.as_str(),
            "old_string": input.old_string.as_str(),
            "new_string": input.new_string.as_str(),
        }),
    );
    exec.add_file_change(file_change);
    let exec = exec.complete_success(
        format!(
            "Applied {} changes (+{} -{}) to {}",
            preview.added + preview.removed,
            preview.added,
            preview.removed,
            input.path
        ),
        duration,
    );
    TOOL_EXECUTIONS.record(exec.success);
    let _ = record_persistent(
        "tool_execution",
        &serde_json::to_value(&exec).unwrap_or_default(),
    );
}
