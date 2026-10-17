//! Retry checks, verification and repair with an inclusive attempt bound.
use super::*;
/// Drive at most `max_quality_retries + 1` quality attempts.
///
/// # Arguments
/// `host` executes callbacks, `state` holds output, and `quality_checks`
/// configures checks. `max_quality_retries` bounds repair attempts.
/// # Returns
/// Updates the supplied state, stopping on verification success or repair failure.
///
/// # Examples
/// ```rust,no_run
/// use codetether_ralph_core::{QualityChecks, quality_repair::{run_quality_retries, RepairHost, QualityProgress}};
/// async fn example(host: &impl RepairHost, checks: &QualityChecks) {
/// let mut state = QualityProgress { output: "initial".into(), qg_passed: false, failure_reason: None };
/// let _ = run_quality_retries(host, &mut state, checks, 2).await;
/// }
/// ```
pub async fn run_quality_retries(
    host: &impl RepairHost,
    state: &mut QualityProgress,
    quality_checks: &QualityChecks,
    max_quality_retries: usize,
) {
    for qg_attempt in 0..=max_quality_retries {
        let (all_ok, mut error_output) = host.check(quality_checks).await;
        if all_ok {
            if host.verify(state, &mut error_output, qg_attempt).await {
                break;
            }
        }

        if qg_attempt < max_quality_retries {
            if !host
                .repair(state, &error_output, qg_attempt, max_quality_retries)
                .await
            {
                break;
            }
        } else {
            host.exhausted(state, &error_output, max_quality_retries);
        }
    }
}
