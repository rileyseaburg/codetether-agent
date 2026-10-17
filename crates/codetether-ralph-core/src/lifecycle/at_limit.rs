//! at_limit terminal-state classification.
use crate::{RalphState, RalphStatus};
use tracing::{info, warn};
pub(super) fn at_limit(state: &mut RalphState, passed: usize, total: usize) {
    if state.current_iteration > 0 && passed == 0 && total > 0 {
        // All stories failed - likely due to quality check failures
        state.status = RalphStatus::QualityFailed;
        warn!(
            iterations = state.current_iteration,
            passed = passed,
            total = total,
            "Ralph failed: all stories failed quality checks"
        );
    } else {
        // Some stories passed but not all
        state.status = RalphStatus::MaxIterations;
        info!(
            iterations = state.current_iteration,
            passed = passed,
            total = total,
            "Ralph finished with partial progress"
        );
    }
}
