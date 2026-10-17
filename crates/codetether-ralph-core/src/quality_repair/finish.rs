//! Resolve an initial model result through host quality and verification checks.
use super::*;
/// Finish one story result, retaining model errors without invoking checks.
///
/// # Arguments
/// `host` provides runtime callbacks; `call_result` is the initial output.
/// `quality_checks_enabled`, `quality_checks` and `max_quality_retries` select retries.
/// # Returns
/// The final model output or error, verified status and optional failure reason.
///
/// # Examples
/// ```rust,no_run
/// use codetether_ralph_core::{QualityChecks, quality_repair::{finish, RepairHost, QualityProgress}};
/// async fn example(host: &impl RepairHost, checks: &QualityChecks) {
/// let mut state = QualityProgress { output: "initial".into(), qg_passed: false, failure_reason: None };
/// let _ = finish(host, Ok("initial".into()), true, checks, 2).await;
/// }
/// ```
pub async fn finish(
    host: &impl RepairHost,
    call_result: Result<String>,
    quality_checks_enabled: bool,
    quality_checks: &QualityChecks,
    max_quality_retries: usize,
) -> (Result<String>, bool, Option<String>) {
    match call_result {
        Ok(initial_output) => {
            let mut state = QualityProgress {
                output: initial_output,
                qg_passed: false,
                failure_reason: None,
            };
            if quality_checks_enabled {
                run_quality_retries(host, &mut state, quality_checks, max_quality_retries).await;
            } else {
                host.verify_without_quality(&mut state).await;
            }
            (Ok(state.output), state.qg_passed, state.failure_reason)
        }
        Err(e) => (Err(e), false, None),
    }
}
