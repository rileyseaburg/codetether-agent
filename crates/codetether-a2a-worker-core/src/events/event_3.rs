//! Structured event regression case.
use super::*;

#[test]
fn tool_result_reports_failure() {
    let event = tool_result_event("bash", false, "boom", None);

    assert_eq!(event["payload"]["status"], serde_json::json!("err"));
    assert_eq!(event["payload"]["success"], serde_json::json!(false));
    assert!(event["payload"].get("duration_ms").is_none());
}
