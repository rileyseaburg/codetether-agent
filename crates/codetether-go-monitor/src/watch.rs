//! Read one pipeline or list active runs without advancing execution.

use crate::{ACTIVE_GO_RUNS, GoRunPhase};
use anyhow::Result;
use codetether_tool_core::ToolResult;
use serde_json::json;
/// Render progress for an optional OKR identifier.
///
/// # Arguments
/// * `okr_id` — A nonempty ID selects one run; absent or blank IDs list all runs.
///
/// # Returns
/// The tool response, including the current phase metadata for a selected run.
///
/// # Errors
/// This implementation returns tool-level errors for unknown run IDs.
///
/// # Examples
/// ```no_run
/// let response = codetether_go_monitor::watch(None).unwrap();
/// assert!(response.success);
/// ```

pub fn watch(okr_id: Option<String>) -> Result<ToolResult> {
    let okr_id_str = match okr_id {
        Some(id) if !id.trim().is_empty() => id,
        _ => {
            // If no OKR ID given, list all active runs
            return crate::list::all();
        }
    };

    // Look up the active run
    let active_run = {
        let runs = ACTIVE_GO_RUNS.lock().unwrap_or_else(|e| e.into_inner());
        runs.get(&okr_id_str).cloned()
    };

    let Some(run) = active_run else {
        return Ok(ToolResult::error(format!(
            "No active go pipeline found for OKR `{okr_id_str}`.\n\n\
                 Use `go(action: \"watch\")` with no okr_id to list active pipelines,\n\
                 or `go(action: \"status\", okr_id: \"...\")` to check completed runs."
        )));
    };

    let mut output = crate::header::render(&run);
    // Phase
    crate::phase_display::append(&mut output, &run);
    // Read PRD file for story-level progress
    crate::stories::append(&mut output, &run);
    // Read progress file
    crate::progress::append(&mut output, &run);
    // Hint for next action
    if matches!(run.phase, GoRunPhase::Starting | GoRunPhase::Running) {
        output.push_str(&format!(
                "\n---\n*Pipeline still running. Call `go(action: \"watch\", okr_id: \"{}\")` again to check progress.*\n",
                run.okr_id
            ));
    }

    Ok(ToolResult::success(output)
        .with_metadata("okr_id", json!(run.okr_id))
        .with_metadata(
            "phase",
            json!(serde_json::to_value(&run.phase).unwrap_or(json!("unknown"))),
        ))
}
