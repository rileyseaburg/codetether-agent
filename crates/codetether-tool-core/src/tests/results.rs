//! Result serialization, structured failures, and truncation metadata.
use crate::ToolResult;
use serde_json::json;

#[test]
fn legacy_results_default_metadata_and_updates_replace_keys() {
    let result: ToolResult = serde_json::from_value(json!({"output":"ok","success":true})).unwrap();
    assert!(result.metadata.is_empty());
    let result = result
        .with_metadata("value", json!(1))
        .with_metadata("value", json!(2));
    assert_eq!(
        serde_json::to_value(result).unwrap(),
        json!({
            "output":"ok", "success":true, "metadata":{"value":2}
        })
    );
}

#[test]
fn truncation_preserves_utf8_and_requested_budget_metadata() {
    let result = ToolResult::success("é🦀end")
        .with_metadata("keep", json!(true))
        .truncate_to(3);
    assert_eq!(result.output, "é\n…[truncated: 9 bytes, showing first 3]");
    assert_eq!(
        result.metadata["truncated"],
        json!({"original_bytes":9,"shown_bytes":3})
    );
    assert_eq!(result.metadata["keep"], true);
    let unchanged = ToolResult::error("error")
        .with_metadata("reason", json!(1))
        .truncate_to(5);
    assert_eq!(unchanged.output, "error");
    assert!(!unchanged.success);
    assert!(!unchanged.metadata.contains_key("truncated"));
}
