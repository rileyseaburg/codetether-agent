//! exhausted terminal-state classification.
use crate::{RalphState, RalphStatus};
use tracing::{info, warn};
pub(super) fn exhausted(state: &mut RalphState, passed: usize, total: usize) {
    // Work completed but execution ended before full completion and before max
    // iteration budget (for example, parallel stages exhausted).
    if passed == 0 && total > 0 {
        state.status = RalphStatus::QualityFailed;
        warn!(
            iterations = state.current_iteration,
            passed = passed,
            total = total,
            "Ralph ended with no passing stories"
        );
    } else {
        state.status = RalphStatus::MaxIterations;
        info!(
            iterations = state.current_iteration,
            passed = passed,
            total = total,
            "Ralph ended before full completion"
        );
    }
}
