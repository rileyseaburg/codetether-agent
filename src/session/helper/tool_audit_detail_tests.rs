use super::{tool_failure_detail, tool_success_detail};
use crate::tool::ToolResult;
use serde_json::json;

#[test]
fn success_detail_includes_hash_preview_and_metadata_keys() {
    let result = ToolResult::success("abcdef").with_metadata("fault_code", json!("x"));
    let detail = tool_success_detail(7, &result);
    assert_eq!(detail["output_len"], 6);
    assert_eq!(detail["output_preview"], "abcdef");
    assert_eq!(detail["metadata_keys"][0], "fault_code");
    assert_eq!(detail["output_sha256"].as_str().unwrap().len(), 64);
}

#[test]
fn failure_detail_avoids_unbounded_error_body() {
    let detail = tool_failure_detail(1, &"x".repeat(700));
    assert_eq!(detail["error_len"], 700);
    assert!(detail["error_preview"].as_str().unwrap().len() < 700);
    assert_eq!(detail["error_sha256"].as_str().unwrap().len(), 64);
}
