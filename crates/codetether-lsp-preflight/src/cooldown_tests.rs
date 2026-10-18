//! Backoff is isolated by workspace and language and expires without extending on reads.

use super::*;

#[test]
fn failed_server_is_skipped_until_retry_deadline() {
    let workspace = tempfile::tempdir().unwrap();
    let other = tempfile::tempdir().unwrap();
    record(workspace.path(), "typescript", "no publication within 5s");
    assert!(
        reason(workspace.path(), "typescript")
            .unwrap()
            .contains("no publication")
    );
    assert!(reason(workspace.path(), "rust").is_none());
    assert!(reason(other.path(), "typescript").is_none());
    assert!(reason(workspace.path(), "typescript").is_some());
    let key = (workspace.path().to_path_buf(), "typescript".into());
    store::entries().get_mut(&key).unwrap().0 = Instant::now() - Duration::from_secs(1);
    assert!(reason(workspace.path(), "typescript").is_none());
}

#[test]
fn pending_work_is_single_flight_and_cleared_on_completion() {
    let root = tempfile::tempdir().unwrap();
    begin(root.path(), "typescript").unwrap();
    assert!(begin(root.path(), "typescript").is_err());
    assert!(
        reason(root.path(), "typescript")
            .unwrap()
            .contains("still running")
    );
    clear(root.path(), "typescript");
    begin(root.path(), "typescript").unwrap();
    clear(root.path(), "typescript");
}
