//! Structured session event construction and redaction.

use super::*;

#[test]
fn tool_call_carries_name_and_arguments() {
    let event = tool_call_event("bash", &serde_json::json!({"command": "ls -la"}));

    assert_eq!(event["type"], serde_json::json!("tool.call"));
    assert_eq!(event["payload"]["name"], serde_json::json!("bash"));
    assert_eq!(
        event["payload"]["arguments"]["command"],
        serde_json::json!("ls -la")
    );
}

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

#[test]
fn tool_result_reports_failure() {
    let event = tool_result_event("bash", false, "boom", None);

    assert_eq!(event["payload"]["status"], serde_json::json!("err"));
    assert_eq!(event["payload"]["success"], serde_json::json!(false));
    assert!(event["payload"].get("duration_ms").is_none());
}

#[test]
fn long_output_is_truncated_explicitly() {
    let long = "x".repeat(MAX_OUTPUT_BYTES * 2);
    let event = tool_result_event("bash", true, &long, None);

    assert_eq!(
        event["payload"]["output_truncated"],
        serde_json::json!(true)
    );
    assert_eq!(
        event["payload"]["output_bytes"],
        serde_json::json!(long.len())
    );
    let carried = event["payload"]["output"].as_str().unwrap();
    assert!(carried.len() <= MAX_OUTPUT_BYTES, "{}", carried.len());
}

#[test]
fn secret_arguments_are_redacted() {
    let event = tool_call_event(
        "bash",
        &serde_json::json!({
            "command": "deploy",
            "GITHUB_TOKEN": "ghp_realsecret",
            "nested": {"api_key": "sk-live"},
        }),
    );
    let arguments = &event["payload"]["arguments"];

    assert_eq!(arguments["GITHUB_TOKEN"], serde_json::json!("[redacted]"));
    assert_eq!(
        arguments["nested"]["api_key"],
        serde_json::json!("[redacted]")
    );
    assert_eq!(arguments["command"], serde_json::json!("deploy"));
}

#[test]
fn secrets_inside_arrays_are_redacted() {
    let event = tool_call_event(
        "bash",
        &serde_json::json!({"items": [{"password": "hunter2"}]}),
    );

    assert_eq!(
        event["payload"]["arguments"]["items"][0]["password"],
        serde_json::json!("[redacted]")
    );
}

#[test]
fn oversized_arguments_are_bounded() {
    let big = "y".repeat(MAX_ARG_BYTES * 2);
    let event = tool_call_event("write", &serde_json::json!({"content": big}));
    let carried = event["payload"]["arguments"]["content"].as_str().unwrap();

    assert!(carried.contains("[truncated]"));
    assert!(carried.len() <= MAX_ARG_BYTES + 32, "{}", carried.len());
}

#[test]
fn sensitive_key_detection_is_case_insensitive() {
    assert!(is_sensitive_key("GITHUB_TOKEN"));
    assert!(is_sensitive_key("Authorization"));
    assert!(is_sensitive_key("db_password"));
    assert!(!is_sensitive_key("path"));
    assert!(!is_sensitive_key("command"));
}

#[test]
fn reasoning_is_its_own_event_type() {
    let event = reasoning_event("Checking the failing test first.");

    assert_eq!(event["type"], serde_json::json!("agent.reasoning"));
    assert_eq!(
        event["payload"]["content"],
        serde_json::json!("Checking the failing test first.")
    );
    assert_eq!(event["payload"]["role"], serde_json::json!("assistant"));
}

#[test]
fn multibyte_output_truncation_stays_valid() {
    let text = "é".repeat(MAX_OUTPUT_BYTES);
    let event = tool_result_event("read", true, &text, None);

    // Serializing proves the carried string is still valid UTF-8.
    assert!(serde_json::to_string(&event).is_ok());
    assert_eq!(
        event["payload"]["output_truncated"],
        serde_json::json!(true)
    );
}
