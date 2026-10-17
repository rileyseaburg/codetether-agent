//! Apply quality results to execution evidence.
use super::ExecutionOutcome;
/// Attach quality output, treating a failed quality invocation as unsuccessful.
///
/// # Arguments
/// `detail` is the engine summary; `changed_files` contains execution evidence;
/// `quality_result` contains the host quality-check result.
/// # Returns
/// Combined evidence with unsuccessful status if the quality invocation failed.
/// # Examples
/// ```
/// let outcome = codetether_forage_core::execution::with_quality(
///     "done".into(), vec![], Ok(("passed".into(), true)));
/// assert!(outcome.quality_gates_passed);
/// ```
pub fn with_quality(
    detail: String,
    changed_files: Vec<String>,
    quality_result: anyhow::Result<(String, bool)>,
) -> ExecutionOutcome {
    let (final_detail, quality_passed) = match quality_result {
        Ok((qr, passed)) => (format!("{}\n\nQuality gates: {}", detail, qr), passed),
        Err(e) => {
            tracing::warn!(target: "codetether_agent::forage", error = %e, "Quality gates failed to run");
            (detail, false) // Treat execution errors as quality failure
        }
    };

    ExecutionOutcome {
        detail: final_detail,
        changed_files,
        quality_gates_passed: quality_passed,
    }
}
