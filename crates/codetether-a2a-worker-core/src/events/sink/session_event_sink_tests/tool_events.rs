//! Tool event types and backwards-compatible text rendering.

use super::*;

#[test]
fn tool_call_and_result_are_emitted_with_types() {
    let _guard = isolated_sink();
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
    let _guard = isolated_sink();
    let (seen, sink) = capture();
    install_sink(Some(sink));
    emit_tool_call("read", &serde_json::json!({}));
    emit_tool_result("read", false, "boom", None);
    install_sink(None);
    let items = seen.lock().unwrap();
    assert_eq!(items.len(), 2);
    assert_eq!(items[0].0, "[tool:start:read]");
    assert!(items[1].0.starts_with("[tool:read:err] "), "{}", items[1].0);
}
