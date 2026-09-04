//! Typed reasoning events and blank-content suppression.

use super::*;

#[test]
fn reasoning_is_emitted_as_its_own_type() {
    let _guard = isolated_sink();
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
    let _guard = isolated_sink();
    let (seen, sink) = capture();
    install_sink(Some(sink));
    emit_reasoning("   \n  ");
    install_sink(None);
    assert!(seen.lock().unwrap().is_empty());
}
