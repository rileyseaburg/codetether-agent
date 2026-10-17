//! Structured event regression case.
use super::*;

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
