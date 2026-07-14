//! Structured result payload for a completed direct swarm invocation.

use super::task_result::TaskResult;
use serde_json::{Value, json};

pub(super) fn build(results: Vec<TaskResult>, failures: usize, strategy: &str) -> Value {
    let total = results.len();
    let successes = results.iter().filter(|result| result.success).count();
    let status = status(failures, strategy);
    json!({
        "status": status,
        "results": results,
        "summary": {
            "total": total,
            "success": successes,
            "failures": failures,
        }
    })
}

fn status(failures: usize, strategy: &str) -> &'static str {
    match (failures, strategy) {
        (0, _) => "success",
        (_, "all") => "partial_failure",
        (_, "first_error") => "error",
        (_, _) => "partial_success",
    }
}

#[cfg(test)]
mod tests {
    use super::status;

    #[test]
    fn failure_status_respects_the_requested_strategy() {
        assert_eq!(status(0, "all"), "success");
        assert_eq!(status(1, "all"), "partial_failure");
        assert_eq!(status(1, "first_error"), "error");
        assert_eq!(status(1, "best_effort"), "partial_success");
    }
}
