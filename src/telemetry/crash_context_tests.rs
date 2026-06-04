//! Tests for crash-report runtime context.

use std::path::Path;

#[test]
fn record_tui_updates_snapshot() {
    super::crash_context::record_tui(
        "session-123",
        42,
        Some("model-x"),
        Some(Path::new("/tmp/work")),
        "Streaming reply…",
    );
    let snap = super::crash_context::snapshot();
    assert_eq!(snap.session_id.as_deref(), Some("session-123"));
    assert_eq!(snap.session_messages, Some(42));
    assert_eq!(snap.session_model.as_deref(), Some("model-x"));
    assert_eq!(snap.cwd.as_deref(), Some("/tmp/work"));
    assert_eq!(snap.tui_status.as_deref(), Some("Streaming reply…"));
}
