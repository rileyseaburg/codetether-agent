//! Human-readable aggregation of subtask results.

use crate::swarm::SubTaskResult;

#[cfg(test)]
#[path = "aggregate_tests.rs"]
mod tests;

pub(super) fn results(results: &[SubTaskResult]) -> String {
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
