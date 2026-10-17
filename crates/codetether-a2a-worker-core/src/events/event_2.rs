//! Structured event regression case.
use super::*;

#[test]
fn tool_result_reports_success_and_status() {
    let event = tool_result_event("read", true, "file contents", Some(42));

    assert_eq!(event["type"], serde_json::json!("tool.result"));
    assert_eq!(event["payload"]["status"], serde_json::json!("ok"));
    assert_eq!(event["payload"]["success"], serde_json::json!(true));
    assert_eq!(event["payload"]["duration_ms"], serde_json::json!(42));
    assert_eq!(
        event["payload"]["output_truncated"],
        serde_json::json!(false)
    );
}
