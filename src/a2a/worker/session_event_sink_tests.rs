//! Task-scoped event sink installation and emission.

use std::sync::{Arc, Mutex};

use super::*;

/// Captures emitted events for assertion.
fn capture() -> (Arc<Mutex<Vec<(String, serde_json::Value)>>>, EventSink) {
    let seen = Arc::new(Mutex::new(Vec::new()));
    let sink_seen = seen.clone();
    let sink: EventSink = Arc::new(move |text, event| {
        if let Some(event) = event
            && let Ok(mut items) = sink_seen.lock()
        {
            items.push((text, event));
        }
    });
    (seen, sink)
}

#[test]
fn tool_call_and_result_are_emitted_with_types() {
    let (seen, sink) = capture();
    install_sink(Some(sink));

    emit_tool_call("bash", &serde_json::json!({"command": "ls"}));
    emit_tool_result("bash", true, "total 12", Some(7));

    install_sink(None);
    let items = seen.lock().unwrap();
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].1["type"], serde_json::json!("tool.call"));
    assert_eq!(items[1].1["type"], serde_json::json!("tool.result"));
    assert_eq!(items[1].1["payload"]["duration_ms"], serde_json::json!(7));
}

#[test]
fn legacy_text_shape_is_preserved() {
    let (seen, sink) = capture();
    install_sink(Some(sink));

    emit_tool_call("read", &serde_json::json!({}));
    emit_tool_result("read", false, "boom", None);

    install_sink(None);
    let items = seen.lock().unwrap();
    assert_eq!(items[0].0, "[tool:start:read]");
    assert!(items[1].0.starts_with("[tool:read:err] "), "{}", items[1].0);
}

#[test]
fn reasoning_is_emitted_as_its_own_type() {
    let (seen, sink) = capture();
    install_sink(Some(sink));

    emit_reasoning("  Checking the failing test first.  ");

    install_sink(None);
    let items = seen.lock().unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].1["type"], serde_json::json!("agent.reasoning"));
    assert_eq!(
        items[0].1["payload"]["content"],
        serde_json::json!("Checking the failing test first.")
    );
}

#[test]
fn blank_reasoning_is_not_emitted() {
    let (seen, sink) = capture();
    install_sink(Some(sink));

    emit_reasoning("   \n  ");

    install_sink(None);
    assert!(seen.lock().unwrap().is_empty());
}

#[test]
fn emitting_without_a_sink_is_a_no_op() {
    install_sink(None);

    // Must not panic when no worker sink is installed, for example under the
    // TUI or `codetether run`.
    emit_tool_call("bash", &serde_json::json!({}));
    emit_tool_result("bash", true, "ok", None);
    emit_reasoning("thinking");
}

#[test]
fn installing_replaces_the_previous_sink() {
    let (first, first_sink) = capture();
    let (second, second_sink) = capture();

    install_sink(Some(first_sink));
    install_sink(Some(second_sink));
    emit_tool_call("bash", &serde_json::json!({}));
    install_sink(None);

    assert!(first.lock().unwrap().is_empty());
    assert_eq!(second.lock().unwrap().len(), 1);
}

#[test]
fn secrets_in_arguments_are_redacted_before_emission() {
    let (seen, sink) = capture();
    install_sink(Some(sink));

    emit_tool_call(
        "bash",
        &serde_json::json!({"command": "deploy", "API_KEY": "sk-live"}),
    );

    install_sink(None);
    let items = seen.lock().unwrap();
    let arguments = &items[0].1["payload"]["arguments"];
    assert_eq!(arguments["API_KEY"], serde_json::json!("[redacted]"));
    assert_eq!(arguments["command"], serde_json::json!("deploy"));
}
