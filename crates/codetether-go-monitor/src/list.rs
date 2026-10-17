//! Render the current in-memory pipeline registry.

use crate::{ACTIVE_GO_RUNS, GoRunPhase};
use anyhow::Result;
use codetether_tool_core::ToolResult;
pub(crate) fn all() -> Result<ToolResult> {
    let runs = ACTIVE_GO_RUNS.lock().unwrap_or_else(|e| e.into_inner());
    if runs.is_empty() {
        return Ok(ToolResult::success(
            "No active go pipelines. Use `go(action: \"execute\", task: \"...\")` to start one.",
        ));
    }

    let mut output = String::from("# Active Go Pipelines\n\n");
    for (id, run) in runs.iter() {
        let phase_str = match &run.phase {
            GoRunPhase::Starting => "Starting".to_string(),
            GoRunPhase::Running => "Running".to_string(),
            GoRunPhase::Completed { passed, total, .. } => {
                format!("Completed ({passed}/{total} passed)")
            }
            GoRunPhase::Failed { error } => {
                format!(
                    "Failed: {}",
                    codetether_util::truncate_bytes_safe(&error, 80)
                )
            }
        };
        output.push_str(&format!(
            "- **{id}**: {phase_str}\n  Task: {}\n  Started: {}\n\n",
            run.task, run.started_at
        ));
    }
    output.push_str("Use `go(action: \"watch\", okr_id: \"<id>\")` for details.");
    Ok(ToolResult::success(output))
}
