//! Detection of prompt steps that make no file-writing progress.

use super::super::super::loop_constants as limits;
use super::super::Runner;
type ToolCall = (String, String, serde_json::Value);

/// Tracks file-writing calls and nudges stalled build agents.
pub(super) fn track_writes(runner: &mut Runner<'_>, step: usize, calls: &[ToolCall]) {
    let wrote = calls.iter().any(|(_, name, _)| {
        matches!(
            name.as_str(),
            "write" | "edit" | "create_file" | "replace_string_in_file" | "edit_file" | "bash"
        )
    });
    if wrote {
        runner.progress.steps_since_write = 0;
    } else {
        runner.progress.steps_since_write += 1;
    }
    if runner.progress.steps_since_write < limits::MAX_STEPS_WITHOUT_PROGRESS {
        return;
    }
    tracing::warn!(
        step,
        steps_since_last_write = runner.progress.steps_since_write,
        "No file-writing tools called recently"
    );
    super::nudge::add(runner, limits::NO_PROGRESS_NUDGE);
    runner.progress.steps_since_write = 0;
}
