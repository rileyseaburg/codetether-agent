//! Bounded progress credit based on engine and verified execution evidence.
use super::ExecutionEvidence;
use codetether_okr::KeyResult;

/// Return bounded fractional progress credit for an incomplete key result.
///
/// # Arguments
/// * `kr` — Key result whose target and current values determine remaining work.
/// # Returns
/// Zero for non-positive targets, otherwise remaining credit clamped to 0.05–0.15.
pub fn success_progress_increment_ratio(kr: &KeyResult) -> f64 {
    if kr.target_value <= 0.0 {
        return 0.0;
    }
    let remaining_ratio = ((kr.target_value - kr.current_value) / kr.target_value).clamp(0.0, 1.0);
    (remaining_ratio * 0.25).clamp(0.05, 0.15)
}
pub(super) fn apply(kr: &mut KeyResult, evidence: &ExecutionEvidence<'_>) -> (bool, f64, f64) {
    let enforce_concrete_file_evidence = evidence.engine == "swarm" || evidence.engine == "go";
    let has_file_evidence = !evidence.changed_files.is_empty();
    let quality_gates_passed = evidence.quality_gates_passed;

    // Only increment progress if:
    // 1. File evidence exists (or not required for "run" engine)
    // 2. Quality gates passed (cargo check/test succeeded)
    let should_increment =
        (has_file_evidence || !enforce_concrete_file_evidence) && quality_gates_passed;

    let before_progress = kr.progress();
    if should_increment {
        let increment_ratio = success_progress_increment_ratio(kr);
        if kr.target_value > 0.0 && increment_ratio > 0.0 {
            let increment_value = (kr.target_value * increment_ratio).max(0.0);
            let new_value = (kr.current_value + increment_value).min(kr.target_value);
            kr.update_progress(new_value);
        }
    }
    let after_progress = kr.progress();

    (should_increment, before_progress, after_progress)
}
