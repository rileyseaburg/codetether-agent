use super::should_sync;

#[test]
fn worker_roster_syncs_only_when_bus_cursor_changes() {
    assert!(should_sync(None, 0));
    assert!(!should_sync(Some(7), 7));
    assert!(should_sync(Some(7), 8));
}
