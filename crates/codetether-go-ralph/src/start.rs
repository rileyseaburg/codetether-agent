//! Start the objective run and record its PRD checkpoint.

use codetether_okr::{OkrRun, OkrRunStatus};
pub(crate) fn mark(okr_run: &mut OkrRun, prd_filename: &str) {
    if let Err(e) = okr_run.start() {
        tracing::warn!(target: "codetether_agent::cli::go_ralph",error = %e, "OKR run start transition failed, forcing Running status");
        okr_run.status = OkrRunStatus::Running;
    }
    okr_run.relay_checkpoint_id = Some(prd_filename.to_owned());
}
