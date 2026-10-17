//! Human-readable aggregation of subtask results.

use crate::SubTaskResult;

#[cfg(test)]
#[path = "aggregate_tests.rs"]
mod tests;

/// Render results in execution order with success and failure labels.
///
/// # Arguments
/// * `results` - Subtask outcomes to render.
///
/// # Returns
/// The parent-facing combined output.
///
/// # Examples
/// ```
/// assert_eq!(codetether_swarm_core::run_policy::aggregate_results(&[]), "");
/// ```
pub fn aggregate_results(results: &[SubTaskResult]) -> String {
    let mut output = String::new();
    for (index, result) in results.iter().enumerate() {
        if result.success {
            output.push_str(&format!(
                "=== Subtask {} ===\n{}\n\n",
                index + 1,
                result.result
            ));
        } else {
            let error = result.error.as_deref().unwrap_or("Unknown error");
            output.push_str(&format!(
                "=== Subtask {} (FAILED) ===\nError: {error}\n\n",
                index + 1
            ));
        }
    }
    output
}
