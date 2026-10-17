//! Select a terminal status after successful scheduling.
use crate::{RalphState, RalphStatus};
/// Set final status from story completion and consumed iterations.
///
/// # Arguments
/// * `state` - Current execution state to classify in place.
/// # Returns
/// Unit; updates status and logs the original completion classification.
/// # Examples
/// ```
/// # fn example(state: &mut codetether_ralph_core::RalphState) {
/// codetether_ralph_core::lifecycle::finalize_status(state);
/// assert_ne!(state.status, codetether_ralph_core::RalphStatus::Running);
/// # }
/// ```
pub fn finalize_status(state: &mut RalphState) {
    // Set final status based on what happened.
    // Parallel mode can exhaust stages before hitting max_iterations, so avoid leaving
    // a terminal run in "Running".
    if state.status != RalphStatus::Completed {
        let passed = state.prd.passed_count();
        let total = state.prd.user_stories.len();

        if state.prd.is_complete() {
            state.status = RalphStatus::Completed;
        } else if state.current_iteration >= state.max_iterations {
            super::at_limit::at_limit(state, passed, total);
        } else if state.current_iteration > 0 {
            super::exhausted::exhausted(state, passed, total);
        } else {
            state.status = RalphStatus::MaxIterations;
        }
    }
}
