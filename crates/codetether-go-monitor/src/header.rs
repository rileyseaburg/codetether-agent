//! Render identifying metadata for one pipeline.

use crate::ActiveGoRun;
pub(crate) fn render(run: &ActiveGoRun) -> String {
    format!(
        "# Go Pipeline Status\n\n\
             **OKR ID:** `{}`\n\
             **Task:** {}\n\
             **Model:** {}\n\
             **Started:** {}\n\
             **Working Directory:** {}\n",
        run.okr_id, run.task, run.model, run.started_at, run.working_dir
    )
}
