//! Map the structured swarm aggregate into a truthful tool outcome.

use crate::tool::ToolResult;
use serde_json::Value;

pub(super) fn result(response: Value, failures: usize) -> ToolResult {
    let output = response.to_string();
    if failures == 0 {
        ToolResult::success(output)
    } else {
        ToolResult::error(output)
    }
}

#[cfg(test)]
mod tests {
    use super::result;

    #[test]
    fn partial_swarm_failure_marks_the_tool_failed() {
        assert!(!result(serde_json::json!({"status": "partial_failure"}), 1).success);
    }
}
