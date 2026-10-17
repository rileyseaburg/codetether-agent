//! Render the callback notification from the shared final state.

use codetether_go_monitor::{ACTIVE_GO_RUNS, GoRunPhase};
pub(crate) fn completed(bg_okr_id: &str) -> String {
    let phase_str = {
        let runs = ACTIVE_GO_RUNS.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(r) = runs.get(bg_okr_id) {
            if let GoRunPhase::Completed {
                passed,
                total,
                all_passed,
                ref feature_branch,
                ref summary,
            } = r.phase
            {
                format!(
                    "# Go Pipeline Completed {}\n\n\
                                         **OKR ID:** `{}`\n\
                                         **Result:** {}/{} stories passed\n\
                                         **Branch:** `{}`\n\n{}",
                    if all_passed { "✅" } else { "⚠️" },
                    bg_okr_id,
                    passed,
                    total,
                    feature_branch,
                    summary
                )
            } else {
                format!("Go pipeline completed for OKR `{}`", bg_okr_id)
            }
        } else {
            format!("Go pipeline completed for OKR `{}`", bg_okr_id)
        }
    };
    phase_str
}
