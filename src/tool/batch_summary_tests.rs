use super::build;
use crate::tool::ToolResult;
use serde_json::json;

#[test]
fn preserves_child_metadata() {
    let child =
        ToolResult::error("blocked").with_metadata("approval_request_id", json!("approval-1"));
    let result = build(vec![(0, "bash".to_string(), child)]);
    assert!(!result.success);
    assert_eq!(result.metadata["error_count"], 1);
    assert_eq!(
        result.metadata["calls"][0]["metadata"]["approval_request_id"],
        "approval-1"
    );
}
