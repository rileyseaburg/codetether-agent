//! IndexedDB persistence tests.

use crate::browser_agent::BrowserContext;

#[test]
fn context_snapshot_restores_indexed_db_records() {
    let mut context = BrowserContext::new();
    context.indexed_db_put("https://example.test/a", "app", "settings", "theme", "dark");

    let snapshot = context.snapshot_state();
    let mut restored = BrowserContext::new();
    restored.restore_state(snapshot).unwrap();

    assert_eq!(
        restored.indexed_db_get("https://example.test/b", "app", "settings", "theme"),
        Some("dark".into())
    );
}
