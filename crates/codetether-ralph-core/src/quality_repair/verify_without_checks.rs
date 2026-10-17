//! Apply verification results when configured quality commands are disabled.
use super::*;
use std::future::Future;
/// Await host verification and update the quality evidence.
///
/// # Arguments
/// `state` receives evidence; `verification` performs the host check.
/// # Returns
/// Updates success and failure fields without changing model output.
/// # Examples
/// ```
/// # async fn example() {
/// use codetether_ralph_core::quality_repair::{QualityProgress, verify_without_checks};
/// let mut state = QualityProgress { output: String::new(), qg_passed: false, failure_reason: None };
/// verify_without_checks(&mut state, async { Ok(()) }).await;
/// assert!(state.qg_passed);
/// # }
/// ```
pub async fn verify_without_checks(
    state: &mut QualityProgress,
    verification: impl Future<Output = Result<()>>,
) {
    match verification.await {
        Ok(()) => {
            state.qg_passed = true;
            state.failure_reason = None;
        }
        Err(e) => {
            state.failure_reason = Some(format!("Verification steps failed:\n{e}"));
        }
    }
}
