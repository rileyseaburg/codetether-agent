//! Map terminal stories to objective outcomes and build the pipeline result.

use crate::GoRalphResult;
use anyhow::Result;
use codetether_okr::{Okr, OkrRun};
use codetether_ralph_core::RalphState;
use std::path::PathBuf;
pub(crate) async fn complete(
    okr: &mut Okr,
    okr_run: &mut OkrRun,
    state: RalphState,
    prd_path: PathBuf,
) -> Result<GoRalphResult> {
    let stories = crate::stories::collect(&state);
    let passed = state.prd.passed_count();
    let total = state.prd.user_stories.len();

    crate::outcomes::map_stories_to_kr_outcomes(okr, okr_run, &state.prd, &state);
    let all_passed = okr.is_complete() || passed == total;

    // Step 7: Update run status
    crate::terminal::mark(okr_run, &state, all_passed);
    // Step 8: Audit-log the result
    crate::audit_completed::record(okr, okr_run, &state, &prd_path, passed, total, all_passed)
        .await;
    Ok(GoRalphResult {
        prd_path,
        feature_branch: state.prd.branch_name.clone(),
        passed,
        total,
        all_passed,
        iterations: state.current_iteration,
        max_iterations: state.max_iterations,
        status: state.status,
        stories,
    })
}
