//! Map terminal Ralph status into objective run status.

use codetether_okr::{OkrRun, OkrRunStatus};
use codetether_ralph_core::{RalphState, RalphStatus};
pub(crate) fn mark(okr_run: &mut OkrRun, state: &RalphState, all_passed: bool) {
    if all_passed {
        okr_run.complete();
    } else if state.status == RalphStatus::Stopped || state.status == RalphStatus::QualityFailed {
        okr_run.status = OkrRunStatus::Failed;
    } else {
        okr_run.status = OkrRunStatus::Completed;
    }
    okr_run.iterations = state.current_iteration as u32;
    okr_run.relay_checkpoint_id = None; // lifecycle complete
}
