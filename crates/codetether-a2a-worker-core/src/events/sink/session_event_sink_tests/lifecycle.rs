//! Sink absence, replacement, and capture isolation.
use super::*;

#[test]
fn emitting_without_a_sink_is_a_no_op() {
    let _guard = isolated_sink();
    emit_tool_call("bash", &serde_json::json!({}));
    emit_tool_result("bash", true, "ok", None);
    emit_reasoning("thinking");
}

#[test]
fn installing_replaces_the_previous_sink() {
    let _guard = isolated_sink();
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
fn capture_ignores_unrelated_test_threads() {
    let _guard = isolated_sink();
    let (seen, sink) = capture();
    install_sink(Some(sink));
    std::thread::spawn(|| {
        emit_tool_call("unrelated", &serde_json::json!({}));
    })
    .join()
    .unwrap();
    emit_tool_call("read", &serde_json::json!({}));
    install_sink(None);
    let items = seen.lock().unwrap();
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].0, "[tool:start:read]");
}
