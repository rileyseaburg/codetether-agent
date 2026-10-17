//! write telemetry for core file tools.
use super::*;
pub(super) fn record(
    path: &str,
    content: &str,
    existed: bool,
    old_content: &Option<String>,
    duration: std::time::Duration,
) {
    // Record telemetry
    let file_change = if existed {
        FileChange::modify(
            path,
            old_content.as_deref().unwrap_or(""),
            content,
            Some((1, content.lines().count() as u32)),
        )
    } else {
        FileChange::create(path, content)
    };

    let mut exec = ToolExecution::start(
        "write",
        json!({
            "path": path,
            "content_length": content.len(),
        }),
    );
    exec.add_file_change(file_change);
    let exec = exec.complete_success(
        format!("Wrote {} bytes to {}", content.len(), path),
        duration,
    );
    TOOL_EXECUTIONS.record(exec.success);
    let _ = record_persistent(
        "tool_execution",
        &serde_json::to_value(&exec).unwrap_or_default(),
    );
}
