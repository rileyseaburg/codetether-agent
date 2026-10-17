//! read telemetry for core file tools.
use super::*;
pub(super) fn record(
    path: &str,
    offset: Option<usize>,
    limit: Option<usize>,
    start_line: usize,
    end_line: usize,
    start: Instant,
) {
    // Record telemetry
    let file_change = FileChange::read(path, Some((start_line as u32 + 1, end_line as u32)));

    let mut exec = ToolExecution::start(
        "read",
        json!({
            "path": path,
            "offset": offset,
            "limit": limit,
        }),
    );
    exec.add_file_change(file_change);
    let exec = exec.complete_success(
        format!("Read {} lines from {}", end_line - start_line, path),
        start.elapsed(),
    );
    TOOL_EXECUTIONS.record(exec.success);
    let _ = record_persistent(
        "tool_execution",
        &serde_json::to_value(&exec).unwrap_or_default(),
    );
}
