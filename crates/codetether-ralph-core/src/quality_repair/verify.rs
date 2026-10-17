//! Apply verification outcomes while the caller supplies verification and reporting.
use super::*;
use std::future::Future;
/// Apply an awaited verification result to the current attempt.
///
/// # Arguments
/// `state` and `error_output` receive evidence; `verification` performs the host
/// check, and `on_success` runs after successful state updates.
/// # Returns
/// Whether verification succeeded; failures retain their formatted diagnostics.
/// # Examples
/// ```
/// # async fn example() {
/// use codetether_ralph_core::quality_repair::{QualityProgress, verify_attempt};
/// let mut state = QualityProgress { output: String::new(), qg_passed: false, failure_reason: None };
/// let mut error = String::new();
/// assert!(verify_attempt(&mut state, &mut error, async { Ok(()) }, || {}).await);
/// # }
/// ```
pub async fn verify_attempt(
    state: &mut QualityProgress,
    error_output: &mut String,
    verification: impl Future<Output = Result<()>>,
    on_success: impl FnOnce(),
) -> bool {
    match verification.await {
        Ok(()) => {
            state.qg_passed = true;
            state.failure_reason = None;
            on_success();
            return true;
        }
        Err(e) => {
            *error_output = format!("Verification steps failed:\n{e}");
            state.failure_reason = Some(error_output.clone());
        }
    }
    false
}
