//! Redaction of secrets before structured tool-call emission.

use super::*;

#[test]
fn secrets_in_arguments_are_redacted_before_emission() {
    let _guard = isolated_sink();
    let (seen, sink) = capture();
    install_sink(Some(sink));
    emit_tool_call(
        "bash",
        &serde_json::json!({"command": "deploy", "API_KEY": "sk-live"}),
    );
    install_sink(None);
    let items = seen.lock().unwrap();
    assert_eq!(items.len(), 1);
    let arguments = &items[0].1["payload"]["arguments"];
    assert_eq!(arguments["API_KEY"], serde_json::json!("[redacted]"));
    assert_eq!(arguments["command"], serde_json::json!("deploy"));
}
